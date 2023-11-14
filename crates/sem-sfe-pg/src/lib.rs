mod parser;
mod pg;
mod pg_to_pbe;

use pg::PG;
use sem_sfe_algorithm::{
    algorithm::{LocalAlgorithm, Player},
    ast::symbolic_exists_moves::SymbolicExistsMoves as UncomposedMoves,
    ast::symbolic_moves_dag::SymbolicExistsMoves,
    normalizer::normalize_system,
};
use sem_sfe_common::{InputFlags, PreProcOutput, SpecOutput, VerificationOutput};

use rustc_hash::FxHashMap as HashMap;

pub struct ParityGameSpec {
    pg: PG,
    node: String,
    position: usize,
}

impl ParityGameSpec {
    pub fn new(src: &mut std::io::BufReader<std::fs::File>, node: String) -> ParityGameSpec {
        let mut pg = parser::parse_pg(src).unwrap();
        pg.0.sort_by(|a, b| a.0.parity.partial_cmp(&b.0.parity).unwrap());

        let position =
            pg.0.iter()
                .enumerate()
                .find_map(|(i, x)| if x.0.name == node { Some(i) } else { None })
                .unwrap_or_else(|| panic!("Cannot find node with name {}", node));

        ParityGameSpec { pg, node, position }
    }
}

impl SpecOutput for ParityGameSpec {
    fn verify(
        &self,
        flags: &InputFlags,
        pre_proc: &PreProcOutput,
    ) -> Result<VerificationOutput, Box<dyn std::error::Error>> {
        let index = if flags.normalize {
            pre_proc
                .fix_system
                .iter()
                .enumerate()
                .find_map(|(i, fix_eq)| {
                    if pre_proc.var_map.get(&pre_proc.var).unwrap() == &fix_eq.var {
                        Some(i + 1)
                    } else {
                        None
                    }
                })
                .unwrap()
        } else {
            self.position + 1
        };

        let algo =
            LocalAlgorithm { fix_system: &pre_proc.fix_system, symbolic_moves: &pre_proc.moves };

        let start = std::time::Instant::now();
        let winner = algo.local_check("true".to_string(), index);
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
            normalize_system(fix_system)
        } else {
            (fix_system, HashMap::default())
        };
        let composed_system = SymbolicExistsMoves::new(
            &fix_system.0,
            &UncomposedMoves {
                basis_map: vec![("true".to_owned(), 0)].into_iter().collect::<HashMap<_, _>>(),
                fun_map: HashMap::default(),
                formulas: Vec::default(),
            },
            &basis,
        );
        let preproc_duration = start.elapsed();

        Ok(PreProcOutput {
            moves: composed_system,
            fix_system: fix_system.0,
            var_map: fix_system.1,
            var: var_name,
            preproc_time: preproc_duration,
        })
    }
}
