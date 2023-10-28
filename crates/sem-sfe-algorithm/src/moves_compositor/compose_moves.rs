use super::simplify;
use crate::ast::fixpoint_system::{ExpFixEq, FixEq};
use crate::ast::symbolic_exists_moves::{
    LogicFormula, SymbolicExistsMove, SymbolicExistsMoveComposed,
};

/// Takes a fixpoint system E, and a collection of symbolic exists-moves
/// for the operators in E. The output are the symbolic exists-moves for
/// the system E.
///
/// *Precondition*: the input has already been sanitized, the fixpoint system
/// contains only operators that have a definition amongst the
/// symbolic $\exists$-moves.
///
pub fn compose_moves(
    e: &Vec<FixEq>,
    s: &Vec<SymbolicExistsMove>,
    basis: &Vec<String>,
) -> Vec<SymbolicExistsMoveComposed> {
    e.iter()
        .enumerate()
        .map(|(i, _)| {
            compose_move_eq(e, i, s, basis)
                .into_iter()
                .map(
                    |SymbolicExistsMove {
                         formula,
                         func_name,
                         basis_elem: base_elem,
                     }| {
                        SymbolicExistsMoveComposed {
                            formula: simplify::simplify(&formula),
                            func_name: func_name.parse().unwrap(),
                            basis_elem: base_elem,
                        }
                    },
                )
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

/// Produces the moves for an expression of the fixpoint system, one for each
/// element of the base.
#[inline]
fn compose_move_eq(
    system: &Vec<FixEq>,
    i: usize,
    s: &Vec<SymbolicExistsMove>,
    basis: &Vec<String>,
) -> Vec<SymbolicExistsMove> {
    basis
        .iter()
        .map(|b| SymbolicExistsMove {
            formula: compose_move_base(system, b, &system[i].exp, s),
            func_name: (i + 1).to_string(),
            basis_elem: b.to_owned(),
        })
        .collect::<Vec<_>>()
}

/// Precondition: the input has been sanitized, meaning, for each possible
///
#[inline]
fn projection(f: &Vec<FixEq>, var: &String) -> usize {
    f.iter()
        .enumerate()
        .map(|(i, x)| (x.var.clone(), i + 1))
        .find(|(x, _)| x == var)
        .unwrap()
        .1
}

/// Output: the composed move for an expression of the system of fixpoint
/// equation, on a single element of the basis.
#[inline]
fn compose_move_base(
    system: &Vec<FixEq>,
    basis_elem: &String,
    sub_exp: &ExpFixEq,
    s: &Vec<SymbolicExistsMove>,
) -> LogicFormula {
    match sub_exp {
        i @ ExpFixEq::And(_, _) => LogicFormula::Conj(vec![
            subst(
                system,
                i,
                s,
                &LogicFormula::BasisElem(basis_elem.to_owned(), 1),
            ),
            subst(
                system,
                i,
                s,
                &LogicFormula::BasisElem(basis_elem.to_owned(), 2),
            ),
        ]),
        i @ ExpFixEq::Or(_, _) => LogicFormula::Disj(vec![
            subst(
                system,
                i,
                s,
                &LogicFormula::BasisElem(basis_elem.to_owned(), 1),
            ),
            subst(
                system,
                i,
                s,
                &LogicFormula::BasisElem(basis_elem.to_owned(), 2),
            ),
        ]),

        i @ ExpFixEq::Operator(op, _) => s
            .iter()
            .find(|SymbolicExistsMove { func_name, basis_elem: b, .. }| {
                func_name == op && b == basis_elem
            })
            .map(|SymbolicExistsMove { formula, .. }| {
                subst(system, &i, s, formula)
            })
            .unwrap()
            .to_owned(),

        ExpFixEq::Id(var) => LogicFormula::BasisElem(
            basis_elem.clone(),
            projection(system, &var),
        ),
    }
}

/// suppose output is sanitized, meaning, for each definition of operator
/// in the symbolic exists-moves, there is the corresponding operator
/// in the system of fixpoint equation, whose arguments are at least the
/// same as the different occurences of atoms of type [b, j] in the formula.
fn subst(
    f: &Vec<FixEq>,
    sub_exp: &ExpFixEq,
    moves: &Vec<SymbolicExistsMove>,
    curr_formula: &LogicFormula,
) -> LogicFormula {
    match curr_formula {
        LogicFormula::BasisElem(b, i) => {
            compose_move_base(f, b, &get_args(sub_exp)[i - 1], moves)
        }
        LogicFormula::Conj(x) => LogicFormula::Conj(
            x.iter().map(|a| subst(f, sub_exp, moves, a)).collect(),
        ),
        LogicFormula::Disj(x) => LogicFormula::Disj(
            x.iter().map(|a| subst(f, sub_exp, moves, a)).collect(),
        ),
        _ => curr_formula.to_owned(),
    }
}

/// output the argument of a function
fn get_args(exp: &ExpFixEq) -> Vec<ExpFixEq> {
    match exp {
        ExpFixEq::And(l, r) | ExpFixEq::Or(l, r) => {
            vec![*l.clone(), *r.clone()]
        }
        ExpFixEq::Operator(_, args) => args.clone(),
        id @ ExpFixEq::Id(_) => vec![id.clone()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::fixpoint_system::{ExpFixEq, FixEq, FixType};
    use crate::ast::symbolic_exists_moves::{
        LogicFormula, SymbolicExistsMove, SymbolicExistsMoveComposed,
    };

    #[test]
    ///
    /// This test takes the following system
    ///
    /// ```
    /// x_1 =max x_2 or box(x_1)
    /// x_2 =min x_1 and diamond(x_2)
    /// ```
    ///
    ///
    /// With basis `{{a}, {b}, {c}, {d}}`.
    /// with provided symbolic exists-moves:
    ///
    /// ```
    /// phi({a})("box")     = [{a}, 1] and [{b}, 1] and [\{c\},1]
    /// phi({b})("box")     = [{c}, 1] and [{d}, 1]
    /// phi({c})("box")     = [{c}, 1]
    /// phi({d})("box")     = [{d}, 1]
    /// phi({a})("diamond") = [{a}, 1] or [{b}, 1] or [{c},1]
    /// phi({b})("diamond") = [{c}, 1] or [{d}, 1]
    /// phi({c})("diamond") = [{c}, 1]
    /// phi({d})("diamond") = [{d}, 1]
    /// ```
    ///
    /// The composition symbolic exists-moves should result
    /// in the following system of moves:
    ///
    /// ```
    /// phi({a})(1) = [{a}, 2] or ([{a}, 1] and [{b}, 1] and [{c},1])
    /// phi({b})(1) = [{b}, 2] or ([{c}, 1] and [{d}, 1])
    /// phi({c})(1) = [{c}, 2] or [{c},1]
    /// phi({d})(1) = [{d}, 2] or [{d},1]
    /// phi({a})(2) = [{a}, 1] and ([{a}, 2] or [{b}, 2] or [{c},2])
    /// phi({b})(2) = [{b}, 1] and ([{c}, 2] or [{d},2])
    /// phi({c})(2) = [{c}, 1] and [{c},2]
    /// phi({d})(2) = [{d}, 1] and [{d},2]
    /// ```
    ///
    fn compose_moves_system() {
        let fix_eq_1 = FixEq {
            var: "x_1".to_string(),
            fix_ty: FixType::Max,
            exp: ExpFixEq::Or(
                Box::new(ExpFixEq::Id("x_2".to_string())),
                Box::new(ExpFixEq::Operator(
                    "box".to_string(),
                    vec![ExpFixEq::Id("x_1".to_string())],
                )),
            ),
        };
        let fix_eq_2 = FixEq {
            var: "x_2".to_string(),
            fix_ty: FixType::Min,
            exp: ExpFixEq::And(
                Box::new(ExpFixEq::Id("x_1".to_string())),
                Box::new(ExpFixEq::Operator(
                    "diamond".to_string(),
                    vec![ExpFixEq::Id("x_2".to_string())],
                )),
            ),
        };

        let formula_box_b = |bs: Vec<&str>, proj: usize| {
            if bs.len() > 1 {
                LogicFormula::Conj(
                    bs.into_iter()
                        .map(|b| LogicFormula::BasisElem(b.to_string(), proj))
                        .collect::<Vec<_>>(),
                )
            } else {
                LogicFormula::BasisElem(bs[0].to_string(), proj)
            }
        };

        let formula_diamond_b = |bs: Vec<&str>, proj: usize| {
            if bs.len() > 1 {
                LogicFormula::Disj(
                    bs.into_iter()
                        .map(|b| LogicFormula::BasisElem(b.to_string(), proj))
                        .collect::<Vec<_>>(),
                )
            } else {
                LogicFormula::BasisElem(bs[0].to_string(), proj)
            }
        };
        let basis = vec!["{a}", "{b}", "{c}", "{d}"]
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        let moves = vec![
            SymbolicExistsMove {
                formula: formula_box_b(vec!["{a}", "{b}", "{c}"], 1),
                func_name: "box".to_string(),
                basis_elem: "{a}".to_string(),
            },
            SymbolicExistsMove {
                formula: formula_box_b(vec!["{c}", "{d}"], 1),
                func_name: "box".to_string(),
                basis_elem: "{b}".to_string(),
            },
            SymbolicExistsMove {
                formula: formula_box_b(vec!["{c}"], 1),
                func_name: "box".to_string(),
                basis_elem: "{c}".to_string(),
            },
            SymbolicExistsMove {
                formula: formula_box_b(vec!["{d}"], 1),
                func_name: "box".to_string(),
                basis_elem: "{d}".to_string(),
            },
            SymbolicExistsMove {
                formula: formula_diamond_b(vec!["{a}", "{b}", "{c}"], 1),
                func_name: "diamond".to_string(),
                basis_elem: "{a}".to_string(),
            },
            SymbolicExistsMove {
                formula: formula_diamond_b(vec!["{c}", "{d}"], 1),
                func_name: "diamond".to_string(),
                basis_elem: "{b}".to_string(),
            },
            SymbolicExistsMove {
                formula: formula_diamond_b(vec!["{c}"], 1),
                func_name: "diamond".to_string(),
                basis_elem: "{c}".to_string(),
            },
            SymbolicExistsMove {
                formula: formula_diamond_b(vec!["{d}"], 1),
                func_name: "diamond".to_string(),
                basis_elem: "{d}".to_string(),
            },
        ];

        let formula_composed_and =
            |b: &str, bs: Vec<&str>, proj_1: usize, proj_2: usize| {
                LogicFormula::Conj(vec![
                    LogicFormula::BasisElem(b.to_string(), proj_1),
                    formula_diamond_b(bs, proj_2),
                ])
            };

        let formula_composed_or =
            |b: &str, bs: Vec<&str>, proj_1: usize, proj_2: usize| {
                LogicFormula::Disj(vec![
                    LogicFormula::BasisElem(b.to_string(), proj_1),
                    formula_box_b(bs, proj_2),
                ])
            };

        let symbolic_composed_moves = vec![
            SymbolicExistsMoveComposed {
                formula: formula_composed_or(
                    "{a}",
                    vec!["{a}", "{b}", "{c}"],
                    2,
                    1,
                ),
                func_name: 1,
                basis_elem: "{a}".to_string(),
            },
            SymbolicExistsMoveComposed {
                formula: formula_composed_or("{b}", vec!["{c}", "{d}"], 2, 1),
                func_name: 1,
                basis_elem: "{b}".to_string(),
            },
            SymbolicExistsMoveComposed {
                formula: formula_composed_or("{c}", vec!["{c}"], 2, 1),
                func_name: 1,
                basis_elem: "{c}".to_string(),
            },
            SymbolicExistsMoveComposed {
                formula: formula_composed_or("{d}", vec!["{d}"], 2, 1),
                func_name: 1,
                basis_elem: "{d}".to_string(),
            },
            SymbolicExistsMoveComposed {
                formula: formula_composed_and(
                    "{a}",
                    vec!["{a}", "{b}", "{c}"],
                    1,
                    2,
                ),
                func_name: 2,
                basis_elem: "{a}".to_string(),
            },
            SymbolicExistsMoveComposed {
                formula: formula_composed_and("{b}", vec!["{c}", "{d}"], 1, 2),
                func_name: 2,
                basis_elem: "{b}".to_string(),
            },
            SymbolicExistsMoveComposed {
                formula: formula_composed_and("{c}", vec!["{c}"], 1, 2),
                func_name: 2,
                basis_elem: "{c}".to_string(),
            },
            SymbolicExistsMoveComposed {
                formula: formula_composed_and("{d}", vec!["{d}"], 1, 2),
                func_name: 2,
                basis_elem: "{d}".to_string(),
            },
        ];

        assert_eq!(
            compose_moves(&vec![fix_eq_1, fix_eq_2], &moves, &basis),
            symbolic_composed_moves
        )
    }

    #[test]
    fn subst_basic_example() {
        let fix_eq_1 = ExpFixEq::And(
            Box::new(ExpFixEq::Operator("p".to_string(), vec![])),
            Box::new(ExpFixEq::Operator(
                "box".to_string(),
                vec![ExpFixEq::Id("x_1".to_string())],
            )),
        );

        let formula_p_b = LogicFormula::True;
        let formula_box_b = LogicFormula::Conj(vec![
            LogicFormula::BasisElem("{d}".to_string(), 1),
            LogicFormula::BasisElem("{e}".to_string(), 1),
        ]);

        let moves = vec![
            SymbolicExistsMove {
                formula: formula_p_b,
                func_name: "p".to_string(),
                basis_elem: "{b}".to_string(),
            },
            SymbolicExistsMove {
                formula: formula_box_b,
                func_name: "box".to_string(),
                basis_elem: "{b}".to_string(),
            },
        ];

        let fix_eq = vec![
            FixEq {
                var: "x_1".to_string(),
                fix_ty: FixType::Max,
                exp: fix_eq_1.clone(),
            },
            //FixEq {var: "x_2".to_string(), fix_ty: FixType::Min, formula:}
        ];

        assert_eq!(
            compose_move_base(&fix_eq, &"{b}".to_string(), &fix_eq_1, &moves),
            LogicFormula::Conj(vec![
                LogicFormula::True,
                LogicFormula::Conj(vec![
                    LogicFormula::BasisElem("{d}".to_string(), 1),
                    LogicFormula::BasisElem("{e}".to_string(), 1)
                ])
            ])
        )
    }
}
