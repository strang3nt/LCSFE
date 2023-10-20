use chumsky::prelude::*;

use crate::ast::symbolic_exists_moves::{LogicFormula, SymbolicExistsMove};
///
/// Returns a parser for the following grammar:
///
/// ```
/// SymMovList ::= SymMovEq SymMovList ';' | SymMovEq ';'
/// SymMovEq ::= 'phi' '(' ID ')' '(' NUM ')' '=' LogicFormula
/// LogicFormula ::= Disjunction
/// Conjunction ::= Atom ('and' Atom)*
/// Disjunction ::= Conjunction ('or' Conjunction)*
/// Atom ::= '[' ID ',' NUM ']' | 'true' | 'false' | '(' LogicFormula ')'
/// ```
///
/// Where `ID in String` and `true`, `false` are respectively syntactic sugar for an empty conjunction and
/// disjunction.
///
/// > Note that the library `Chumsky`, and in general parser combinators libraries
/// > have a limited support for left recursion.
///
pub fn symbolic_moves_parser(
    fun_with_arities: &Vec<(String, usize)>,
    basis: &Vec<String>,
) -> impl Parser<char, Vec<SymbolicExistsMove>, Error = Simple<char>> {
    let basis =
        basis.iter().map(|str| just(str.clone()).padded()).collect::<Vec<_>>();

    let logic_formula = recursive(|logic_formula| {
        let base_elem = (choice(basis.clone())
            .then_ignore(just(','))
            .then(text::int(10).padded()))
        .delimited_by(just('['), just(']'))
        .map(|(base, int)| LogicFormula::BasisElem(base, int.parse().unwrap()));

        let truth = text::keyword("true").map(|_| LogicFormula::True);
        let falsehood = text::keyword("false").map(|_| LogicFormula::False);

        let atom = base_elem
            .or(truth)
            .or(falsehood)
            .or(logic_formula.delimited_by(just('('), just(')')));

        let op = |c| just(c).padded();

        let and = atom
            .clone()
            .separated_by(op("and"))
            .map(|conj| LogicFormula::Conj(conj));

        let and_or_atom = and.clone().or(atom.clone());

        let or = and_or_atom
            .clone()
            .separated_by(op("or"))
            .map(|disj| LogicFormula::Disj(disj));

        or
    });

    let fun_name = fun_with_arities
        .iter()
        .map(|(str, _)| {
            just(str.clone()).padded().delimited_by(just('('), just(')'))
        })
        .collect::<Vec<_>>();

    let move_eq = just("phi")
        .padded()
        .ignore_then(choice(basis).delimited_by(just('('), just(')')))
        .then(choice(fun_name))
        .then_ignore(just('=').padded())
        .then(logic_formula)
        .map(|((base, fun), formula): ((String, String), LogicFormula)| {
            SymbolicExistsMove {
                formula: formula,
                basis_elem: base,
                func_name: fun,
            }
        });

    let symbolic_move_list =
        move_eq.separated_by(just(';')).allow_trailing().padded();

    symbolic_move_list.then_ignore(end())
}
