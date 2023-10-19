mod parser;
mod pg;
mod pg_to_pbe;

use pg::PG;
use sem_lmc_algorithm::{
    algorithm::{EvePos, ParityGame, Player, Position},
    ast::{fixpoint_system::FixEq, symbolic_exists_moves::SymbolicExistsMove},
    moves_compositor::compose_moves::compose_moves,
};
use sem_lmc_common::SpecOutput;

pub struct ParityGameSpec {
    pg: PG,
    player: bool,
    node: String,
}

impl ParityGameSpec {
    pub fn new(
        src: &mut std::io::BufReader<std::fs::File>,
        player: bool,
        node: String,
    ) -> ParityGameSpec {
        let mut pg = parser::parse_pg(src).unwrap();
        pg.0.sort_by(|a, b| a.0.id.partial_cmp(&b.0.id).unwrap());

        println!("{:#?}", pg);
        ParityGameSpec { pg: pg, player, node }
    }
}

impl SpecOutput for ParityGameSpec {
    fn get_sys(&self) -> Result<Vec<FixEq>, Box<dyn std::error::Error>> {
        let player = match self.player {
            false => pg::Player::Eve,
            true => pg::Player::Adam,
        };

        Ok(pg_to_pbe::pg_to_pbe(&self.pg, player))
    }

    fn get_sem(
        &self,
    ) -> Result<Vec<SymbolicExistsMove>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }

    fn get_ver(&self) -> String {
        let basis = &vec!["true".to_string()];

        let algo = ParityGame {
            fix_system: &self.get_sys().unwrap(),
            symbolic_moves: &compose_moves(
                &self.get_sys().unwrap(),
                &vec![],
                basis,
            ),
            basis,
        };
        let node = self
            .pg
            .0
            .iter()
            .enumerate()
            .find(|(_, x)| x.0.name == self.node)
            .map(|(i, _)| i + 1);

        let winner = match algo.local_check(Position::Eve(EvePos {
            b: "true".to_string(),
            i: node.unwrap(),
        })) {
            Player::Adam => 1,
            Player::Eve => 0,
        };

        format!("Player {} wins from vertex {}", winner, self.node)

    }
}
