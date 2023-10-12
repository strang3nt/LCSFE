mod moves_compositor;
mod parity_game;
mod parser;
use std::env;

use chumsky::prelude::*;
use parity_game::{position::EvePos, position::Position, ParityGame};
use parser::{
    arity_parser, basis_parser, eq_system_parser, moves_parser,
    symbolic_exists_moves::{SymbolicExistsMove, SymbolicExistsMoveComposed},
};

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

    let basis_src =
        std::fs::read_to_string(std::env::args().nth(4).unwrap()).unwrap();
    let basis = basis_parser::basis_parser(basis_src);
    println!("{:?}", basis);

    let parity_game = ParityGame {
        fix_system: &fixpoint_system.unwrap(),
        symbolic_moves: &symbolic_moves
            .unwrap()
            .into_iter()
            .map(|SymbolicExistsMove { formula, func_name, base_elem }| {
                SymbolicExistsMoveComposed {
                    formula,
                    func_name: func_name.parse().unwrap(),
                    base_elem,
                }
            })
            .collect(),
        basis: &basis,
    };

    let winner = parity_game
        .local_check(Position::Eve(EvePos { b: "c".to_string(), i: 2 }));

    print!("{:?}", winner);
}
