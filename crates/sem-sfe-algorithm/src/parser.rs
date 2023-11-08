use std::fmt;

mod arity_parser;
mod basis_parser;
mod eq_system_parser;
mod moves_parser;

#[derive(Debug, Clone)]
pub struct ParserError {
    details: String,
}

impl ParserError {
    pub fn new(details: String) -> ParserError {
        ParserError { details }
    }
}

impl std::error::Error for ParserError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error(s):\n{}", self.details)
    }
}

use crate::ast::fixpoint_system::FixEq;
use crate::ast::symbolic_exists_moves::SymbolicExistsMoves;
use chumsky::prelude::*;

pub fn parse_basis(src: String) -> Result<Vec<String>, Vec<ParserError>> {
    Ok(basis_parser::basis_parser(src))
}

pub fn parse_symbolic_system(
    arity: &[(String, usize)],
    basis: &[String],
    src: String,
) -> Result<SymbolicExistsMoves, ParserError> {
    moves_parser::symbolic_moves_parser(arity, basis).parse(src).map_err(|errs| {
        ParserError::new(
            errs.into_iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\n- "),
        )
    })
}

pub fn parse_fixpoint_system(
    arity: &[(String, usize)],
    src: String,
) -> Result<Vec<FixEq>, ParserError> {
    eq_system_parser::eq_system_parser(arity).parse(src).map_err(|errs| {
        ParserError::new(
            errs.into_iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\n- "),
        )
    })
}

pub fn parse_fun_arity(src: String) -> Result<Vec<(String, usize)>, ParserError> {
    arity_parser::arity_parser().parse(src).map_err(|errs| {
        ParserError::new(
            errs.into_iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\n- "),
        )
    })
}
