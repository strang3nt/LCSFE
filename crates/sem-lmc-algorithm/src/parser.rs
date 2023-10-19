use std::fmt;

mod arity_parser;
mod basis_parser;
mod eq_system_parser;
mod moves_parser;

#[derive(Debug, Clone)]
pub struct ParseError {
    expected: String,
    reason: String,
    span: String,
    found: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "found {} at {}, expected {}",
            self.found, self.span, self.expected
        )
    }
}

use crate::ast::fixpoint_system::FixEq;
use crate::ast::symbolic_exists_moves::SymbolicExistsMove;
use chumsky::prelude::*;

pub fn parse_basis(src: String) -> Result<Vec<String>, ParseError> {
    unimplemented!()
}
pub fn parse_symbolic_system(
    src: String,
) -> Result<Vec<SymbolicExistsMove>, ParseError> {
    unimplemented!()
}
pub fn parse_fixpoint_system(src: String) -> Result<Vec<FixEq>, ParseError> {
    unimplemented!()
}
pub fn parse_fun_arity(
    src: String,
) -> Result<Vec<(String, usize)>, ParseError> {
    unimplemented!()
}
