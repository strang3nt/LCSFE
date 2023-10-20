pub mod moves_compositor;
mod parity_game;
mod parser;

pub mod ast;
pub mod parse {
    pub use crate::parser::parse_basis;
    pub use crate::parser::parse_fixpoint_system;
    pub use crate::parser::parse_fun_arity;
    pub use crate::parser::parse_symbolic_system;
    pub use crate::parser::ParserError;
}

pub mod algorithm {
    pub use crate::parity_game::player::Player;
    pub use crate::parity_game::position::{AdamPos, EvePos, Position};
    pub use crate::parity_game::ParityGame;
}

// use std::env;

// use chumsky::prelude::*;
// use parity_game::{position::EvePos, position::Position, ParityGame};
// use parser::{arity_parser, basis_parser, eq_system_parser, moves_parser};

// use crate::moves_compositor::compose_moves::compose_moves;

// fn main() {
//     let dir = env::current_dir().unwrap();
//     print!("\n\n{:?}\n\n", dir);

//     print!(
//         "Arguments are: {:?} \n",
//         std::env::args().into_iter().collect::<Vec<String>>()
//     );

//     let arity_src =
//         std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

//     let arity_vec = arity_parser::arity_parser().parse(arity_src).unwrap();
//     println!("{:?}", arity_vec);

//     let fixpoint_system_src =
//         std::fs::read_to_string(std::env::args().nth(2).unwrap()).unwrap();
//     let fixpoint_system = eq_system_parser::eq_system_parser(&arity_vec)
//         .parse(fixpoint_system_src)
//         .unwrap();

//     println!("{:?}", fixpoint_system);

//     let basis_src =
//         std::fs::read_to_string(std::env::args().nth(3).unwrap()).unwrap();
//     let basis = basis_parser::basis_parser(basis_src);
//     println!("{:?}", basis);

//     let symbolic_moves_src =
//         std::fs::read_to_string(std::env::args().nth(4).unwrap()).unwrap();
//     let symbolic_moves =
//         moves_parser::symbolic_moves_parser(&arity_vec, &basis)
//             .parse(symbolic_moves_src);
//     println!("{:?}", symbolic_moves);

//     let parity_game = ParityGame {
//         fix_system: &fixpoint_system,
//         symbolic_moves: &compose_moves(
//             &fixpoint_system,
//             &symbolic_moves.unwrap(),
//             &basis,
//         ),
//         basis: &basis,
//     };

//     println!("{:?}", parity_game.symbolic_moves);

//     let winner = parity_game
//         .local_check(Position::Eve(EvePos { b: "{e}".to_string(), i: 2 }));

//     print!("{:?}", winner);
// }
