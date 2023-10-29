mod parser;
mod pg;
mod pg_to_pbe;

use std::collections::HashMap;

use pg::PG;
use sem_sfe_algorithm::{
    algorithm::{EvePos, LocalAlgorithm, Player, Position},
    moves_compositor::compose_moves,
    normalizer::normalize_system,
};
use sem_sfe_common::{InputFlags, SpecOutput, VerificationOutput, PreProcOutput};

pub struct ParityGameSpec {
    pg: PG,
    node: String,
    position: usize,
}

impl ParityGameSpec {
    pub fn new(
        src: &mut std::io::BufReader<std::fs::File>,
        node: String,
    ) -> ParityGameSpec {

        let mut pg = parser::parse_pg(src).unwrap();
        pg.0.sort_by(|a, b| a.0.parity.partial_cmp(&b.0.parity).unwrap());

        let position = pg.0.iter()
            .enumerate()
            .find_map(|(i, x)| if x.0.name == node { Some(i) } else { None })
            .expect(&format!("Cannot find node with name {}", node));

        ParityGameSpec { pg: pg, node, position }
    }
}

impl SpecOutput for ParityGameSpec {
    fn verify(
        &self,
        flags: &InputFlags,
        pre_proc: &PreProcOutput,
    ) -> Result<VerificationOutput, Box<dyn std::error::Error>> {

        let index = if flags.normalize {
            pre_proc.fix_system.iter().enumerate().find_map(|(i, fix_eq)| {
                if pre_proc.var_map.get(&pre_proc.var).unwrap() == &fix_eq.var { Some(i + 1) } else { None }}).unwrap()
        } else { self.position };

        let algo = LocalAlgorithm {
            fix_system: &pre_proc.fix_system,
            symbolic_moves: &pre_proc.moves,
        };

        let start = std::time::Instant::now();
        let winner = algo.local_check(Position::Eve(EvePos{ b: "true".to_string(), i: index}));
        let algo_duration = start.elapsed();

        let winner = match winner {
            Player::Adam => 1,
            Player::Eve => 0,
        };

        Ok(sem_sfe_common::VerificationOutput {
            algorithm_time: algo_duration,
            result: format!("Player {} wins from vertex {}", winner, self.node),
        })
    }

    fn pre_proc(&self, flags: &InputFlags) -> Result<PreProcOutput, Box<dyn std::error::Error>> {
        let basis = vec!["true".to_string()];

        let start = std::time::Instant::now();
        let fix_system = pg_to_pbe::pg_to_pbe(&self.pg, pg::Player::Eve);
        let var_name = fix_system
            .iter()
            .enumerate()
            .find(|(i, _)| i == &self.position)
            .map(|(_, x)| x.var.to_owned())
            .unwrap();
        let fix_system = if flags.normalize {
            normalize_system(&fix_system)
        } else {
            (fix_system, HashMap::new())
        };
        let composed_system = compose_moves::compose_moves(
            &fix_system.0,
            &vec![],
            &basis,
        );
        let preproc_duration = start.elapsed();

        Ok(PreProcOutput { moves: composed_system, fix_system: fix_system.0, var_map: fix_system.1, var: var_name, preproc_time: preproc_duration })
    }
}
