use chumsky::prelude::*;

use crate::parser::symbolic_exists_moves::{SymbolicExistsMove, LogicFormula, SymbolicSystem};

///
/// Returns a parser for the following grammar:
/// 
/// ```
/// SymMovList ::= SymMovEq SymMovList ';' | SymMovEq ';'
/// SymMovEq ::= 'phi' '(' ID ')' '(' NUM ')' '=' LogicFormula
/// LogicFormula ::= Disjunction
/// Conjunction ::= Atom ('and' Atom)*
/// Disjunction ::= Conjunction ('or' Conjunction)*
/// Atom ::= ID | "true" | "false"
/// 
/// ```
/// 
/// Where $ID\in String$ and `true`, `false` are respectively syntactic sugar for an empty conjunction and
/// disjunction. 
/// 
/// > Note that the library `Chumsky`, and in general parser combinators libraries 
/// > have a limited support for left recursion.
/// 
pub fn symbolic_moves_parser() -> impl Parser<char, SymbolicSystem, Error = Simple<char>> {

    let base_elem = 
        (text::ident().padded()
            .then_ignore(just(',').padded())
            .then(text::int(10)))
            .delimited_by(just('['), just(']'))
            .map(|(base, int)| LogicFormula::BaseElem(base, int.parse().unwrap()));

    let truth = text::keyword("true").map(|_| LogicFormula::True);
    let falsehood = text::keyword("false").map(|_| LogicFormula::False);

    let atom = 
        base_elem.or(truth).or(falsehood);

    let op = |c| just(c).padded();

    let and = 
        atom.clone()
            .separated_by(op("and"))
            .map(|conj| LogicFormula::Conj(conj));

    let or = and.clone()
        .separated_by(op("or"))
        .map(|disj| LogicFormula::Disj(disj));

    let move_eq = 
        just("phi")
            .padded()
            .ignore_then(text::int(10).padded().delimited_by(just('('), just(')')))
            .then(text::ident().padded().delimited_by(just('('), just(')')))
            .then_ignore(just('=').padded())
            .then(or)
            .map(| ((fun, base), formula)| SymbolicExistsMove {
                formula: formula,
                base_elem: base,
                func_name: fun.parse().unwrap(),
            });

    let symbolic_move_list = 
        move_eq.clone()
            .separated_by(just(';')).allow_trailing().padded()
            .map(|eq| SymbolicSystem(eq) );

    symbolic_move_list.then_ignore(end())

}
