mod parser;
mod parity_game;

use chumsky::prelude::*;
use parser::{arity_parser, eq_system_parser, moves_parser};


fn main() {


    let arity_src = std::fs::read_to_string(
        std::env::args()
            .nth(1)
            .unwrap())
        .unwrap();

    let arity_vec = arity_parser::arity_parser().parse(arity_src);
    println!("{:?}", arity_vec);

    let fixpoint_system = std::fs::read_to_string(
        std::env::args()
            .nth(2)
            .unwrap())
        .unwrap();
    println!("{:?}", eq_system_parser::eq_system_parser( &arity_vec.unwrap() ).parse(fixpoint_system));

    let symbolic_moves = std::fs::read_to_string(
        std::env::args()
            .nth(3)
            .unwrap())
        .unwrap();
    println!("{:?}", moves_parser::symbolic_moves_parser().parse(symbolic_moves));
}