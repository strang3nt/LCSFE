mod ald_parser;
mod mu_calc_parser;

use std::{io::Read, time::Instant};
use rustc_hash::FxHashMap as HashMap;
use ald_parser::{ald_parser, Lts};
use chumsky::Parser;
use mu_calc_parser::MuCalc;
use sem_sfe_algorithm::{
    algorithm::{EvePos, LocalAlgorithm, Player},
    moves_compositor::compose_moves::compose_moves,
    normalizer::normalize_system,
};
use sem_sfe_common::{InputFlags, PreProcOutput, SpecOutput, VerificationOutput};

pub struct MuAld {
    lts: Lts,
    formula: MuCalc,
    state: String,
}

impl MuAld {
    pub fn new(
        lts_src: &mut std::io::BufReader<std::fs::File>,
        formula_src: &mut std::io::BufReader<std::fs::File>,
        state: String,
    ) -> Result<MuAld, Box<dyn std::error::Error>> {
        let lts = ald_parser(lts_src)?;
        let mut formula = String::new();
        formula_src.read_to_string(&mut formula).expect("cannot read string");
        let formula = mu_calc_parser::mu_calc_parser(&lts.labels).parse(formula).unwrap();
        Ok(MuAld { lts, formula, state })
    }
}

impl SpecOutput for MuAld {
    fn pre_proc(&self, flags: &InputFlags) -> Result<PreProcOutput, Box<dyn std::error::Error>> {
        let start = Instant::now();
        let (fix_system, moves) = mu_calc_parser::mucalc_to_fix_system(&self.formula, &self.lts)?;
        let (fix_system, var_map) = if flags.normalize {
            normalize_system(fix_system)
        } else {
            (fix_system, HashMap::default())
        };
        let moves = compose_moves(
            &fix_system,
            &moves,
            &self.lts.adj_list.iter().map(|x| x.0.to_string()).collect::<Vec<_>>(),
        );
        let preproc_time = start.elapsed();

        Ok(PreProcOutput { moves, fix_system, var_map, var: self.state.to_owned(), preproc_time })
    }

    fn verify(
        &self,
        _: &InputFlags,
        pre_proc: &PreProcOutput,
    ) -> Result<sem_sfe_common::VerificationOutput, Box<dyn std::error::Error>> {
        let local_algorithm =
            LocalAlgorithm { fix_system: &pre_proc.fix_system, symbolic_moves: &pre_proc.moves };

        let start = Instant::now();
        let result =
            local_algorithm.local_check(sem_sfe_algorithm::algorithm::Position::Eve(EvePos {
                b: self.state.to_owned(),
                i: local_algorithm.fix_system.len(),
            }));
        let algorithm_time = start.elapsed();

        let result = match result {
            Player::Eve => "is satisfied",
            Player::Adam => "is not satisfied",
        };

        let result = format!("The property {} from state {}", result, self.state);
        Ok(VerificationOutput { algorithm_time, result })
    }
}
