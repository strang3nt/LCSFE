use std::{fmt::Display, io::Error};

use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

use chumsky::prelude::*;
use lcsfe_algorithm::ast::{
    fixpoint_system::{ExpFixEq, FixEq, FixType},
    symbolic_moves::{LogicFormula, SymbolicExistsMoves},
};

use crate::ald_parser::Lts;

#[derive(Debug)]
pub enum Act {
    Label(String),
    NotLabel(String),
    True,
}

impl Display for Act {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Act::Label(a) => write!(f, "{}", a),
            Act::True => write!(f, "true"),
            Act::NotLabel(a) => write!(f, "not_{}", a),
        }
    }
}

#[derive(Debug)]
pub enum MuCalc {
    True,
    False,
    Var(String),
    Eta(String, FixType, Box<MuCalc>),
    Diamond(Act, Box<MuCalc>),
    Box(Act, Box<MuCalc>),
    And(Box<MuCalc>, Box<MuCalc>),
    Or(Box<MuCalc>, Box<MuCalc>),
}

fn instantiate_diamond(
    lts: &Lts,
    act: &Act,
    formulas: &mut [LogicFormula],
    basis_map: &HashMap<String, usize>,
    fun_map: &HashMap<String, usize>,
) {
    lts.adj_list.iter().for_each(|(basis_elem, edges)| {
        let mut nodes = edges
            .iter()
            .filter_map(|(l, node)| match act {
                Act::Label(x) if x == &lts.labels[*l] => Some(node),
                Act::True => Some(node),
                Act::NotLabel(x) if x != &lts.labels[*l] => Some(node),
                _ => None,
            })
            .peekable();
        if nodes.peek().is_none() {
            formulas[fun_map.get(&format!("diamond_{}", act)).unwrap() * basis_map.len()
                + basis_map.get(&basis_elem.to_string()).unwrap()] = LogicFormula::False;
        } else {
            formulas[fun_map.get(&format!("diamond_{}", act)).unwrap() * basis_map.len()
                + basis_map.get(&basis_elem.to_string()).unwrap()] = LogicFormula::Disj(
                nodes
                    .map(|n| LogicFormula::BasisElem(n.to_string(), 0))
                    .collect::<Vec<_>>(),
            );
        }
    })
}

fn instantiate_box(
    lts: &Lts,
    act: &Act,
    formulas: &mut [LogicFormula],
    basis_map: &HashMap<String, usize>,
    fun_map: &HashMap<String, usize>,
) {
    lts.adj_list.iter().for_each(|(basis_elem, edges)| {
        let mut nodes = edges
            .iter()
            .filter_map(|(l, node)| match act {
                Act::Label(x) if x == &lts.labels[*l] => Some(node),
                Act::True => Some(node),
                Act::NotLabel(x) if x != &lts.labels[*l] => Some(node),
                _ => None,
            })
            .peekable();
        if nodes.peek().is_none() {
            formulas[fun_map.get(&format!("box_{}", act)).unwrap() * basis_map.len()
                + basis_map.get(&basis_elem.to_string()).unwrap()] = LogicFormula::True;
        } else {
            formulas[fun_map.get(&format!("box_{}", act)).unwrap() * basis_map.len()
                + basis_map.get(&basis_elem.to_string()).unwrap()] = LogicFormula::Conj(
                nodes
                    .map(|n| LogicFormula::BasisElem(n.to_string(), 0))
                    .collect::<Vec<_>>(),
            );
        }
    })
}

pub fn mucalc_to_fix_system(
    formula: &MuCalc,
    lts: &Lts,
) -> Result<(Vec<FixEq>, SymbolicExistsMoves), Error> {
    match &formula {
        MuCalc::Eta(_, _, _) => {
            let (var_counter, foos) = preproc_formula(formula);
            let basis_map = lts
                .adj_list
                .iter()
                .enumerate()
                .map(|(i, x)| (x.0.to_string(), i))
                .collect::<HashMap<String, usize>>();

            let fun_map = foos
                .into_iter()
                .enumerate()
                .map(|(i, x)| (x, i))
                .collect::<HashMap<String, usize>>();
            let mut formulas = vec![LogicFormula::False; basis_map.len() * fun_map.len()];
            let (_, fix_sys) = get_fix_system(
                formula,
                lts,
                &basis_map,
                &fun_map,
                &mut formulas,
                var_counter,
                &mut HashMap::default(),
            );
            Ok((
                fix_sys,
                SymbolicExistsMoves {
                    basis_map,
                    fun_map,
                    formulas,
                },
            ))
        }
        _ => Err(Error::new(
            std::io::ErrorKind::Other,
            "The input formula is not a fixpoint formula",
        )),
    }
}

/// in one visit I want to know:
///
///  - how many new variables I have to instantiate
///  - respectively, if there is a box, diamond, true, false syntax elements.
///
/// The goal is to know the shape of the formula, in order to build the fixpoint
/// system and to build the SymbolicExistsMoves struct.
fn preproc_formula(formula: &MuCalc) -> (u32, HashSet<String>) {
    match formula {
        MuCalc::True => {
            let mut foos = HashSet::default();
            foos.insert("tt".to_owned());
            (0, foos)
        }
        MuCalc::False => {
            let mut foos = HashSet::default();
            foos.insert("ff".to_owned());
            (0, foos)
        }
        MuCalc::Var(_) => (0, HashSet::default()),
        MuCalc::Eta(_, _, e) => {
            let (i, foos) = preproc_formula(e);
            (i + 1, foos)
        }
        MuCalc::Diamond(a, e) => {
            let (i, mut foos) = preproc_formula(e);
            foos.insert(format!("diamond_{}", a));
            (i, foos)
        }
        MuCalc::Box(a, e) => {
            let (i, mut foos) = preproc_formula(e);
            foos.insert(format!("box_{}", a));
            (i, foos)
        }
        MuCalc::And(l, r) | MuCalc::Or(l, r) => {
            let (li, mut lfoos) = preproc_formula(l);
            let (ri, rfoos) = preproc_formula(r);
            lfoos.extend(rfoos);
            (li + ri, lfoos)
        }
    }
}

fn get_fix_system(
    formula: &MuCalc,
    lts: &Lts,
    basis_map: &HashMap<String, usize>,
    fun_map: &HashMap<String, usize>,
    formulas: &mut Vec<LogicFormula>,
    mut var_counter: u32,
    var_map: &mut HashMap<String, String>,
) -> (ExpFixEq, Vec<FixEq>) {
    match formula {
        MuCalc::Eta(x, fix_ty, e) => {
            let x_i = format!("x_{}", var_counter);
            var_counter -= 1;
            var_map.insert(x.to_string(), x_i.clone());
            let (exp, mut system) =
                get_fix_system(e, lts, basis_map, fun_map, formulas, var_counter, var_map);
            system.push(FixEq {
                var: x_i.clone(),
                fix_ty: fix_ty.clone(),
                exp,
            });
            (ExpFixEq::Id(x_i), system)
        }
        MuCalc::Diamond(a, e) => {
            let (exp, system) =
                get_fix_system(e, lts, basis_map, fun_map, formulas, var_counter, var_map);
            instantiate_diamond(lts, a, formulas, basis_map, fun_map);
            (
                ExpFixEq::Operator(format!("diamond_{}", a), vec![exp]),
                system,
            )
        }
        MuCalc::Box(a, e) => {
            let (exp, system) =
                get_fix_system(e, lts, basis_map, fun_map, formulas, var_counter, var_map);

            instantiate_box(lts, a, formulas, basis_map, fun_map);
            (ExpFixEq::Operator(format!("box_{}", a), vec![exp]), system)
        }
        MuCalc::And(l, r) => {
            let (lexp, mut lsystem) =
                get_fix_system(l, lts, basis_map, fun_map, formulas, var_counter, var_map);
            let (rexp, rsystem) =
                get_fix_system(r, lts, basis_map, fun_map, formulas, var_counter, var_map);
            lsystem.extend(rsystem);
            (ExpFixEq::And(Box::new(lexp), Box::new(rexp)), lsystem)
        }
        MuCalc::Or(l, r) => {
            let (lexp, mut lsystem) =
                get_fix_system(l, lts, basis_map, fun_map, formulas, var_counter, var_map);
            let (rexp, rsystem) =
                get_fix_system(r, lts, basis_map, fun_map, formulas, var_counter, var_map);
            lsystem.extend(rsystem);
            (ExpFixEq::Or(Box::new(lexp), Box::new(rexp)), lsystem)
        }
        MuCalc::Var(x) => (
            ExpFixEq::Id(
                var_map
                    .get(x)
                    .cloned()
                    .unwrap_or_else(|| panic!("Error: variable {} was not declared", x)),
            ),
            vec![],
        ),

        MuCalc::True => {
            lts.adj_list.iter().for_each(|x| {
                formulas[fun_map.get("tt").unwrap() * basis_map.len()
                    + basis_map.get(&x.0.to_string()).unwrap()] = LogicFormula::True;
            });

            (ExpFixEq::Operator("tt".to_string(), vec![]), vec![])
        }
        MuCalc::False => {
            lts.adj_list.iter().for_each(|x| {
                formulas[fun_map.get("ff").unwrap() * basis_map.len()
                    + basis_map.get(&x.0.to_string()).unwrap()] = LogicFormula::False;
            });
            (ExpFixEq::Operator("ff".to_string(), vec![]), vec![])
        }
    }
}

/// A parser for the following grammar:
///
/// <Atom> ::= `tt' | `ff' | `(' <MuCalc> `)'
///         | <Id>
/// <ModalOp> ::= `<' <Label> `>' <Atom>
///         | `[' <Label> `]' <Atom>
///         | <Atom>
/// <Conjunction> ::= <Atom> (`&&' <Atom>)*
/// <Disjuction>  ::= <Conjunction> (`||' <Conjunction>)*
/// <Fix> ::= | `mu' <Id> `.' <Disjunction>
///          | `nu' <Id> `.' <Disjunction>
/// <MuCalc> ::= <Fix> | <Disjunction>
/// <Label> ::= `true' | <Id>
/// <Id> ::= ( a C-style identifier )
///
pub fn mu_calc_parser(labels: &[String]) -> impl Parser<char, MuCalc, Error = Simple<char>> {
    let expr = recursive(|expr| {
        let var = text::ident().map(MuCalc::Var).padded();

        let mut labels_parser = labels
            .iter()
            .map(|str| just(str.clone()).padded())
            .collect::<Vec<_>>();
        labels_parser.push(just("true".to_string()).padded());
        let labels_parser = choice(labels_parser).map(|x| {
            if x == "true" {
                Act::True
            } else {
                Act::Label(x)
            }
        });

        let not_labels = labels
            .iter()
            .map(|x| just('!').padded().ignore_then(just(x.clone())))
            .collect::<Vec<_>>();
        let not_labels = choice(not_labels).map(Act::NotLabel);

        let tt = text::keyword("tt").padded().map(|_| MuCalc::True);
        let ff = text::keyword("ff").padded().map(|_| MuCalc::False);
        let atom = tt
            .or(ff)
            .or(expr.clone().delimited_by(just('('), just(')')))
            .or(var)
            .padded();

        let diamond = choice((labels_parser.clone(), not_labels.clone()))
            .delimited_by(just('<'), just('>'))
            .then(atom.clone())
            .map(|(l, expr)| MuCalc::Diamond(l, Box::new(expr)))
            .padded();
        let boxx = choice((labels_parser, not_labels))
            .delimited_by(just('['), just(']'))
            .then(atom.clone())
            .map(|(l, expr)| MuCalc::Box(l, Box::new(expr)))
            .padded();

        let modalop = diamond.or(boxx).or(atom);
        let op = |c| just(c).padded();
        let and = modalop
            .clone()
            .then(
                op("&&")
                    .to(MuCalc::And as fn(_, _) -> _)
                    .then(modalop)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let or = and
            .clone()
            .then(
                op("||")
                    .to(MuCalc::Or as fn(_, _) -> _)
                    .then(and)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let mu = text::keyword("mu")
            .ignore_then(text::ident().padded())
            .then_ignore(just('.').padded())
            .then(or.clone())
            .map(|(x, e)| MuCalc::Eta(x, FixType::Min, Box::new(e)));
        let nu = text::keyword("nu")
            .ignore_then(text::ident().padded())
            .then_ignore(just('.').padded())
            .then(or.clone())
            .map(|(x, e)| MuCalc::Eta(x, FixType::Max, Box::new(e)));

        mu.or(nu).or(or).padded()
    });

    expr.then_ignore(end())
}
