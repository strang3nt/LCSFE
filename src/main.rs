mod parity_game;
mod parser;

use std::env;

use chumsky::prelude::*;
use fixpoint_system::Eq;
use parity_game::parity_game::{FixpointType, ParityGame};
use parity_game::position::Position;
use parser::{arity_parser, eq_system_parser, fixpoint_system, moves_parser};

fn main() {
    let dir = env::current_dir().unwrap();
    print!("\n\n{:?}\n\n", dir);

    print!(
        "Arguments are: {:?} \n",
        std::env::args().into_iter().collect::<Vec<String>>()
    );

    let arity_src =
        std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    let arity_vec = arity_parser::arity_parser().parse(arity_src);
    println!("{:?}", arity_vec);

    let fixpoint_system_src =
        std::fs::read_to_string(std::env::args().nth(2).unwrap()).unwrap();
    let fixpoint_system =
        eq_system_parser::eq_system_parser(&arity_vec.unwrap())
            .parse(fixpoint_system_src);

    println!("{:?}", fixpoint_system);

    let symbolic_moves_src =
        std::fs::read_to_string(std::env::args().nth(3).unwrap()).unwrap();
    let symbolic_moves =
        moves_parser::symbolic_moves_parser().parse(symbolic_moves_src);
    println!("{:?}", symbolic_moves);

    let mut base: Vec<String> = symbolic_moves
        .clone()
        .unwrap()
        .0
        .clone()
        .iter()
        .map(|sym| sym.base_elem.clone())
        .collect::<Vec<String>>();

    base.sort();
    base.dedup();
    let parity_game = ParityGame {
        fix_types: fixpoint_system
            .unwrap()
            .0
            .iter()
            .map(|eq| match eq {
                Eq::Max(_, _) => FixpointType::Max,
                Eq::Min(_, _) => FixpointType::Min,
            })
            .collect(),
        symbolic_moves: symbolic_moves.unwrap().clone(),
        base: base,
    };

    let winner = parity_game.local_check(Position::Eve("a".to_string(), 2));

    print!("{:?}", winner);
}
