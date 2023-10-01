use crate::parser::fixpoint_system::{Eq, ExpEq, FixpointSystem};
use chumsky::prelude::*;

///
/// Returns a parser for the following grammar:
///
/// ```
/// EqList ::= Eq EqList ';' | Eq ';'
/// Eq ::= ID '=max' ExpEq | ID '=min' ExpEq
/// ExpEq ::= OrExpEq
/// Atom ::= ID | '(' ExpEq ')' | CustomExpEq
/// AndExpEq ::= Atom ('and' Atom)*
/// OrExpEq ::= AndExpEq ('or' AndExpEq)*
/// CustomExpEq ::= OP '(' ExpEq (',' ExpEq)* ')'
/// ```
///
/// As it is, this grammar is able to parse only a normalized system of symbolic
/// equations: each equation is a disjunction of conjunctions.
/// The parser receives an array of characters and if successful returns a type `FixpointSystem`.
/// Notice that the syntactic category `AndExpEq` has a higher precedence
/// than `OrExpEq`, this way we enforce the precedence of the operator $\wedge$ over
/// $\vee$.
/// Tokens `ID` and `OP` are a strings, the latter represents the name of an operator provided as input to the parser.
/// If the goal is to parse $\mu$-calculus formulae, a possible definition for `OP` would be $OP \in \{'diamond', 'box'\}$.
///
/// > Note that the library `Chumsky`, and in general have a limited support for left recursion.
///
pub fn eq_system_parser(
    fun_with_arities: &Vec<(String, usize)>,
) -> impl Parser<char, FixpointSystem, Error = Simple<char>> {
    let expr = recursive(|expr| {
        let var = text::ident().map(ExpEq::Id).padded();

        let fun_arguments = fun_with_arities
            .iter()
            .map(|(str, size)| {
                just(str.clone()).padded().then(
                    expr.clone()
                        .separated_by(just(','))
                        .exactly(size.clone())
                        .delimited_by(just('('), just(')')),
                )
            })
            .collect::<Vec<_>>();

        let custom_op = choice(fun_arguments)
            .map(|(name, args)| ExpEq::CustomOp(name, args));

        let atom = custom_op
            .or(var)
            .or(expr.clone().delimited_by(just('('), just(')')));

        let op = |c| just(c).padded();

        let and = atom
            .clone()
            .then(
                op("and")
                    .to(ExpEq::And as fn(_, _) -> _)
                    .then(atom)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let or = and
            .clone()
            .then(op("or").to(ExpEq::Or as fn(_, _) -> _).then(and).repeated())
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        or
    });

    let fix_type = |c| just(c).padded();

    let equation = text::ident()
        .padded()
        .then(
            fix_type("=max")
                .to(Eq::Max as fn(_, _) -> _)
                .or(fix_type("=min").to(Eq::Min as fn(_, _) -> _)),
        )
        .then(expr.clone())
        .map(|((var, fix_ty), exp_eq)| fix_ty(var, exp_eq));

    let system_of_equations = equation
        .clone()
        .separated_by(just(';'))
        .allow_trailing()
        .padded()
        .map(|eq| FixpointSystem(eq));

    system_of_equations.then_ignore(end())
}
