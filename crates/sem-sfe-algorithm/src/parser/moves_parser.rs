use rustc_hash::FxHashMap as HashMap;

use chumsky::prelude::*;

use crate::ast::symbolic_exists_moves::{LogicFormula, SymbolicExistsMoves};
///
/// Returns a parser for the following grammar:
///
///
/// SymMovList ::= SymMovEq SymMovList ';' | SymMovEq ';'
/// SymMovEq ::= 'phi' '(' ID ')' '(' NUM ')' '=' LogicFormula
/// LogicFormula ::= Disjunction
/// Conjunction ::= Atom ('and' Atom)*
/// Disjunction ::= Conjunction ('or' Conjunction)*
/// Atom ::= '[' ID ',' NUM ']' | 'true' | 'false' | '(' LogicFormula ')'
///
///
/// Where `ID in String` and `true`, `false` are respectively syntactic sugar for an empty conjunction and
/// disjunction.
///
/// > Note that the library `Chumsky`, and in general parser combinators libraries
/// > have a limited support for left recursion.
///
pub fn symbolic_moves_parser<'a>(
    fun_with_arities: &'a [(String, usize)],
    basis: &'a [String],
) -> impl Parser<char, SymbolicExistsMoves, Error = Simple<char>> + 'a {
    let basis_parser = basis.iter().map(|str| just(str.clone()).padded()).collect::<Vec<_>>();

    let logic_formula = recursive(|logic_formula| {
        let base_elem =
            (choice(basis_parser.clone()).then_ignore(just(',')).then(text::int(10).padded()))
                .delimited_by(just('['), just(']'))
                .map(|(base, int)| LogicFormula::BasisElem(base, int.parse().unwrap()));

        let truth = text::keyword("true").map(|_| LogicFormula::True);
        let falsehood = text::keyword("false").map(|_| LogicFormula::False);

        let atom =
            base_elem.or(truth).or(falsehood).or(logic_formula.delimited_by(just('('), just(')')));

        let op = |c| just(c).padded();

        let and = atom.clone().separated_by(op("and")).map(LogicFormula::Conj);

        let and_or_atom = and.clone().or(atom.clone());

        and_or_atom.clone().separated_by(op("or")).map(LogicFormula::Disj)
    });

    let fun_name = fun_with_arities
        .iter()
        .map(|(str, _)| just(str.clone()).padded().delimited_by(just('('), just(')')))
        .collect::<Vec<_>>();

    let move_eq = just("phi")
        .padded()
        .ignore_then(choice(basis_parser).delimited_by(just('('), just(')')))
        .then(choice(fun_name))
        .then_ignore(just('=').padded())
        .then(logic_formula);

    let symbolic_move_list = move_eq.separated_by(just(';')).allow_trailing().padded().map(|x| {
        let basis_map = basis
            .iter()
            .enumerate()
            .map(|(i, b)| (b.to_owned(), i))
            .collect::<HashMap<String, usize>>();
        let fun_map = fun_with_arities
            .iter()
            .enumerate()
            .map(|(i, (f, _))| (f.to_owned(), i))
            .collect::<HashMap<String, usize>>();
        let mut formulas = vec![LogicFormula::False; basis_map.len() * fun_map.len()];
        x.into_iter().for_each(|((b, f), l)| {
            formulas[fun_map.get(&f).unwrap() * basis_map.len() + basis_map.get(&b).unwrap()] = l
        });
        SymbolicExistsMoves { basis_map, fun_map, formulas }
    });

    symbolic_move_list.then_ignore(end())
}
