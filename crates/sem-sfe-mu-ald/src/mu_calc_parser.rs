use std::{collections::HashMap, fmt::Display, io::Error};

use chumsky::prelude::*;
use sem_sfe_algorithm::ast::{
    fixpoint_system::{ExpFixEq, FixEq, FixType},
    symbolic_exists_moves::{LogicFormula, SymbolicExistsMove},
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

fn instantiate_diamond(lts: &Lts, act: &Act) -> Vec<SymbolicExistsMove> {
    lts.adj_list
        .iter()
        .map(|(basis_elem, edges)| {
            let nodes = edges
                .iter()
                .filter_map(|(l, node)| {
                    match act {
                        Act::Label(x) if x == &lts.labels[*l] => Some(node),
                        Act::True => Some(node),
                        Act::NotLabel(x) if x != &lts.labels[*l]  => Some(node),
                        _ => None
                    }
                })
                .collect::<Vec<_>>();
            if nodes.is_empty() {
                SymbolicExistsMove {
                    formula: LogicFormula::True,
                    func_name: format!("diamond_{}", act),
                    basis_elem: basis_elem.to_string(),
                }
            } else {
                SymbolicExistsMove {
                    formula: LogicFormula::Disj(
                        nodes
                            .iter()
                            .enumerate()
                            .map(|(i, n)| {
                                LogicFormula::BasisElem(n.to_string(), i + 1)
                            })
                            .collect::<Vec<_>>(),
                    ),
                    func_name: format!("diamond_{}", act),
                    basis_elem: basis_elem.to_string(),
                }
            }
        })
        .collect::<Vec<_>>()
}

fn instantiate_box(lts: &Lts, act: &Act) -> Vec<SymbolicExistsMove> {
    lts.adj_list
        .iter()
        .map(|(basis_elem, edges)| {
            let nodes = edges
            .iter()
            .filter_map(|(l, node)| {
                match act {
                    Act::Label(x) if x == &lts.labels[*l] => Some(node),
                    Act::True => Some(node),
                    Act::NotLabel(x) if x != &lts.labels[*l]  => Some(node),
                    _ => None
                }
            })
            .collect::<Vec<_>>();
            if nodes.is_empty() {
                SymbolicExistsMove {
                    formula: LogicFormula::False,
                    func_name: format!("box_{}", act),
                    basis_elem: basis_elem.to_string(),
                }
            } else {
                SymbolicExistsMove {
                    formula: LogicFormula::Conj(
                        nodes
                            .iter()
                            .enumerate()
                            .map(|(i, n)| {
                                LogicFormula::BasisElem(n.to_string(), i + 1)
                            })
                            .collect::<Vec<_>>(),
                    ),
                    func_name: format!("box_{}", act),
                    basis_elem: basis_elem.to_string(),
                }
            }
        })
        .collect::<Vec<_>>()
}

pub fn mucalc_to_fix_system(
    formula: &MuCalc,
    lts: &Lts,
) -> Result<(Vec<FixEq>, Vec<SymbolicExistsMove>), Error> {
    print!("{:#?}", formula);
    match &formula {
        
        MuCalc::Eta(_, _, _) => {
            let var_counter = count_vars(formula);
            let (_, fix_sys, moves) = get_fix_system(
                formula,
                lts,
                var_counter,
                &mut HashMap::default(),
            );
            Ok((fix_sys, moves))
        }
        _ => Err(Error::new(
            std::io::ErrorKind::Other,
            "The input formula is not a fixpoint formula",
        )),
    }
}

fn count_vars(formula: &MuCalc) -> u32 {
    match formula {
        MuCalc::True => 0,
        MuCalc::False => 0,
        MuCalc::Var(_) => 0,
        MuCalc::Eta(_, _, e) => 1 + count_vars(e),
        MuCalc::Diamond(_, e) => count_vars(e),
        MuCalc::Box(_, e) => count_vars(e),
        MuCalc::And(l, r) => count_vars(l) + count_vars(r),
        MuCalc::Or(l, r) => count_vars(l) + count_vars(r),
    }
}

fn get_fix_system(
    formula: &MuCalc,
    lts: &Lts,
    mut var_counter: u32,
    var_map: &mut HashMap<String, String>,
) -> (ExpFixEq, Vec<FixEq>, Vec<SymbolicExistsMove>) {
    match formula {
        MuCalc::Eta(x, fix_ty, e) => {
            let x_i = format!("x_{}", var_counter);
            var_counter -= 1;
            var_map.insert(x.to_string(), x_i.clone());
            let (exp, mut system, moves) =
                get_fix_system(e, lts, var_counter, var_map);
            system.push(FixEq {
                var: x_i.clone(),
                fix_ty: fix_ty.clone(),
                exp,
            });
            (ExpFixEq::Id(x_i), system, moves)
        }
        MuCalc::Diamond(a, e) => {
            let (exp, system, mut moves) =
                get_fix_system(e, lts, var_counter, var_map);

            if moves.iter().any(|x| x.func_name == format!("diamond_{}", a)) {
                (
                    ExpFixEq::Operator(format!("diamond_{}", a), vec![exp]),
                    system,
                    moves,
                )
            } else {
                moves.extend(instantiate_diamond(lts, a));
                (
                    ExpFixEq::Operator(format!("diamond_{}", a), vec![exp]),
                    system,
                    moves,
                )
            }
        }
        MuCalc::Box(a, e) => {
            let (exp, system, mut moves) =
                get_fix_system(e, lts, var_counter, var_map);

            if moves.iter().any(|x| x.func_name == format!("box_{}", a)) {
                (
                    ExpFixEq::Operator(format!("box_{}", a), vec![exp]),
                    system,
                    moves,
                )
            } else {
                moves.extend(instantiate_box(lts, a));
                (
                    ExpFixEq::Operator(format!("box_{}", a), vec![exp]),
                    system,
                    moves,
                )
            }
        }
        MuCalc::And(l, r) => {
            let (lexp, mut lsystem, mut lmoves) =
                get_fix_system(l, lts, var_counter, var_map);
            let (rexp, rsystem, rmoves) =
                get_fix_system(r, lts, var_counter, var_map);
            lsystem.extend(rsystem);
            lmoves.extend(rmoves);
            (ExpFixEq::And(Box::new(lexp), Box::new(rexp)), lsystem, lmoves)
        }
        MuCalc::Or(l, r) => {
            let (lexp, mut lsystem, mut lmoves) =
                get_fix_system(l, lts, var_counter, var_map);
            let (rexp, rsystem, rmoves) =
                get_fix_system(r, lts, var_counter, var_map);
            lsystem.extend(rsystem);
            lmoves.extend(rmoves);
            (ExpFixEq::Or(Box::new(lexp), Box::new(rexp)), lsystem, lmoves)
        }
        MuCalc::Var(x) => {
            println!("{}", x);
            (ExpFixEq::Id(var_map.get(x).cloned().unwrap()), vec![], vec![])
        }
        MuCalc::True => (
            ExpFixEq::Operator("tt".to_string(), vec![]),
            vec![],
            lts.adj_list
                .iter()
                .map(|x| SymbolicExistsMove {
                    formula: LogicFormula::True,
                    func_name: "tt".to_string(),
                    basis_elem: x.0.to_string(),
                })
                .collect::<Vec<_>>(),
        ),
        MuCalc::False => (
            ExpFixEq::Operator("ff".to_string(), vec![]),
            vec![],
            lts.adj_list
                .iter()
                .map(|x| SymbolicExistsMove {
                    formula: LogicFormula::False,
                    func_name: "ff".to_string(),
                    basis_elem: x.0.to_string(),
                })
                .collect::<Vec<_>>(),
        ),
    }
}

/// A parser for the following grammar:
///
/// <Atom> ::= `tt' | `ff' | `(' <Disjunction> `)'
///         | <Id>
///         | `<' <Label> `>' <Disjunction>
///         | `[' <Label> `]' <Disjunction>
///         | `mu' <Id> `.' <Disjunction>
///         | `nu' <Id> `.' <Disjunction>
/// <Conjunction> ::= <Atom> (`&&' <Atom>)*
/// <Disjuction>  ::= <Conjunction> (`||' <Conjunction>)*
/// <Label> ::= `true' | <Id>
/// <Id> ::= ( a C-style identifier )
///
pub fn mu_calc_parser(
    labels: &Vec<String>,
) -> impl Parser<char, MuCalc, Error = Simple<char>> {
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

        let exclamation = just::<_, _, Simple<char>>('!');
        let not_labels = labels.iter().map(|x| exclamation.clone().padded().ignore_then(just(x.clone()))).collect::<Vec<_>>();
        let not_labels = choice(not_labels).map(Act::NotLabel);

        let atom = just("tt").map(|_| MuCalc::True).padded()
            .or(just("ff").map(|_| MuCalc::False).padded())
            .or(choice((labels_parser.clone(), not_labels.clone())).delimited_by(just("<"), just(">")).then(expr.clone()).padded().map(|(l, expr)| MuCalc::Diamond(l, Box::new(expr))))
            .or(choice((labels_parser.clone(), not_labels.clone())).delimited_by(just("["), just("]")).then(expr.clone()).padded().map(|(l, expr)| MuCalc::Box(l, Box::new(expr))))
            .or(just("mu").padded().ignore_then(text::ident().padded()).then_ignore(just(".")).then(expr.clone()).map(|(x, e)| MuCalc::Eta(x, FixType::Min, Box::new(e))))
            .or(just("nu").padded().ignore_then(text::ident().padded()).then_ignore(just(".")).then(expr.clone()).map(|(x, e)| MuCalc::Eta(x, FixType::Max, Box::new(e))))
            .or(var)
            .or(expr.clone().delimited_by(just('('), just(')')))
            ;

        let op = |c| just(c).padded();

        let and = atom
            .clone()
            .then(
                op("&&").to(MuCalc::And as fn(_, _) -> _).then(atom).repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        and.clone()
            .then(op("||").to(MuCalc::Or as fn(_, _) -> _).then(and).repeated())
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
    });

    expr.then_ignore(end())
}
