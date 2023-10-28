mod parser;
mod pg;
mod pg_to_pbe;

use pg::PG;
use sem_sfe_algorithm::{
    algorithm::{EvePos, LocalAlgorithm, Player, Position},
    moves_compositor::compose_moves,
    normalizer::normalize_system,
};
use sem_sfe_common::{InputFlags, SpecOutput, VerificationOutput};

pub struct ParityGameSpec {
    pg: PG,
    node: String,
}

impl ParityGameSpec {
    pub fn new(
        src: &mut std::io::BufReader<std::fs::File>,
        node: String,
    ) -> ParityGameSpec {
        let mut pg = parser::parse_pg(src).unwrap();
        pg.0.sort_by(|a, b| a.0.parity.partial_cmp(&b.0.parity).unwrap());

        ParityGameSpec { pg: pg, node }
    }
}

impl SpecOutput for ParityGameSpec {
    fn verify(
        &self,
        flags: &InputFlags,
    ) -> Result<VerificationOutput, Box<dyn std::error::Error>> {
        let basis = vec!["true".to_string()];

        let start = std::time::Instant::now();
        let fix_system = pg_to_pbe::pg_to_pbe(&self.pg, pg::Player::Eve);
        let normalized_system = if flags.normalize {
            Some(normalize_system(&fix_system))
        } else {
            None
        };
        let composed_system = compose_moves::compose_moves(
            if flags.normalize {
                &normalized_system.as_ref().unwrap().0
            } else {
                &fix_system
            },
            &vec![],
            &basis,
        );
        let preproc_duration = start.elapsed();

        let position = self
            .pg
            .0
            .iter()
            .enumerate()
            .find(|(_, x)| x.0.name == self.node)
            .map(|(i, _)| i)
            .expect(&format!("Cannot find variable with name {}", self.node));

        let var_name = fix_system
            .iter()
            .enumerate()
            .find(|(i, _)| i == &position)
            .map(|(_, x)| &x.var)
            .unwrap();

        let index = normalized_system
            .as_ref()
            .map_or(&fix_system, |(x, _)| x)
            .iter()
            .enumerate()
            .find(|(_, fix_eq)| {
                if flags.normalize {
                    normalized_system.as_ref().unwrap().1.get(var_name).expect(
                        &format!("Cannot find variable with name {}", var_name),
                    ) == &fix_eq.var
                } else {
                    var_name == &fix_eq.var
                }
            })
            .map(|(i, _)| i + 1)
            .expect(&format!("Cannot find variable with name {}", var_name));

        let start = std::time::Instant::now();

        println!("Verification starts from variable {:#?}", var_name);
        let algo = LocalAlgorithm {
            fix_system: normalized_system
                .as_ref()
                .map_or(fix_system.as_ref(), |x| &x.0),
            symbolic_moves: &composed_system,
            basis: &basis,
        };

        let winner = algo.local_check(Position::Eve(EvePos {
            b: "true".to_string(),
            i: index,
        }));

        let algo_duration = start.elapsed();

        let winner = match winner {
            Player::Adam => 1,
            Player::Eve => 0,
        };

        Ok(sem_sfe_common::VerificationOutput {
            fix_system: fix_system,
            fix_system_normalized: normalized_system.map(|x| x.0),
            moves_composed: composed_system,
            moves: vec![],
            preproc_time: preproc_duration,
            algorithm_time: algo_duration,
            result: format!("Player {} wins from vertex {}", winner, self.node),
        })
    }
}
