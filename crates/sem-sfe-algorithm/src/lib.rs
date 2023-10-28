mod moves_compositor;
pub use moves_compositor::compose_moves;
mod parity_game;
mod parser;

pub mod ast;
pub mod normalizer;
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
    pub use crate::parity_game::LocalAlgorithm;
}
