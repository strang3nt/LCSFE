mod parser;
mod pg;
mod pg_to_pbe;

use pg::PG;
use sem_lmc_algorithm::{
    algorithm::{EvePos, LocalAlgorithm, Player, Position},
    moves_compositor::compose_moves,
    normalizer::normalize_system,
};
use sem_lmc_common::{InputFlags, SpecOutput, VerificationOutput};

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
        let fix_system = if flags.normalize {
            normalize_system(&fix_system)
        } else {
            fix_system
        };
        let composed_system =
            compose_moves::compose_moves(&fix_system, &vec![], &basis);
        let preproc_duration = start.elapsed();

        let algo = LocalAlgorithm {
            fix_system: &fix_system,
            symbolic_moves: &composed_system,
            basis: &basis,
        };
        let node = self
            .pg
            .0
            .iter()
            .enumerate()
            .find(|(_, x)| x.0.name == self.node)
            .map(|(i, _)| i + 1);

        let start = std::time::Instant::now();
        let winner = algo.local_check(Position::Eve(EvePos {
            b: "true".to_string(),
            i: node.unwrap(),
        }));
        let algo_duration = start.elapsed();

        let winner = match winner {
            Player::Adam => 1,
            Player::Eve => 0,
        };

        Ok(sem_lmc_common::VerificationOutput {
            preproc_time: preproc_duration,
            algorithm_time: algo_duration,
            result: format!("Player {} wins from vertex {}", winner, self.node),
        })
    }
}
