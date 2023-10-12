use super::simplify;
use crate::parser::{
    fixpoint_system,
    symbolic_exists_moves::{self, SymbolicExistsMoveComposed},
};
use fixpoint_system::{ExpFixEq, FixEq};
use symbolic_exists_moves::{LogicFormula, SymbolicExistsMove};

/// Takes a fixpoint system $E$, and a collection of symbolic $\exists$-moves
/// for the operators in $E$. The output are the symbolic $\exists$-moves for
/// the system $E$, that is $(\phi^i_b)_{b\in B_L,i\in \underline m}.
///
/// *Precondition*: the input has already been sanitized, the fixpoint system
/// contains only moves that have a definition amongst the
/// symbolic $\exists$-moves.
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
                .map(|SymbolicExistsMove { formula, func_name, base_elem }| {
                    SymbolicExistsMoveComposed {
                        formula: simplify::simplify(&formula),
                        func_name: func_name.parse().unwrap(),
                        base_elem,
                    }
                })
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
            base_elem: b.clone(),
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
    base_elem: &String,
    sub_exp: &ExpFixEq,
    s: &Vec<SymbolicExistsMove>,
) -> LogicFormula {
    match sub_exp {
        i @ ExpFixEq::And(_, _) | i @ ExpFixEq::Or(_, _) => subst(
            system,
            i,
            base_elem,
            s,
            &LogicFormula::Conj(vec![
                LogicFormula::BaseElem(base_elem.clone(), 1),
                LogicFormula::BaseElem(base_elem.clone(), 2),
            ]),
        ),
        i @ ExpFixEq::CustomOp(op, _) => subst(
            system,
            i,
            base_elem,
            s,
            s.iter()
                .find(|SymbolicExistsMove { func_name, base_elem: b, ..}| 
                    func_name == op && b == base_elem)
                .map(|SymbolicExistsMove { formula, .. }| formula)
                .unwrap(),
        ),

        ExpFixEq::Id(var) => {
            LogicFormula::BaseElem(base_elem.clone(), projection(system, &var))
        }
    }
}

/// suppose output is sanitized, meaning, for each definition of operator
/// in the symbolic existential moves, there is the corresponding operator
/// in the system of fixpoint equation, whose arguments are at least the
/// same as the different occurences of atoms of type [b, j] in the formula.
fn subst(
    f: &Vec<FixEq>,
    sub_exp: &ExpFixEq,
    base_elem: &String,
    moves: &Vec<SymbolicExistsMove>,
    curr_formula: &LogicFormula,
) -> LogicFormula {
    match curr_formula {
        LogicFormula::BaseElem(b, i) => match &get_args(sub_exp)[i - 1] {
            ExpFixEq::And(l, r) => LogicFormula::Conj(vec![
                subst(f, &l, base_elem, moves, &LogicFormula::BaseElem(b.clone(), 1)),
                subst(f, &r, base_elem, moves, &LogicFormula::BaseElem(b.clone(), 2)),
            ]),
            ExpFixEq::Or(l, r) => LogicFormula::Disj(vec![
                subst(f, &l, base_elem, moves, &LogicFormula::BaseElem(b.clone(), 1)),
                subst(f, &r, base_elem, moves, &LogicFormula::BaseElem(b.clone(), 2)),
            ]),
            i @ ExpFixEq::CustomOp(name, _) => moves
                .iter()
                .find(|SymbolicExistsMove { func_name, base_elem: b, ..}| 
                    func_name == name && b == base_elem)
                .map(|SymbolicExistsMove { formula, .. }| {
                    subst(f, &i, base_elem, moves, formula)
                })
                .unwrap()
                .clone(),
            ExpFixEq::Id(var) => {
                LogicFormula::BaseElem(b.clone(), projection(f, &var))
            }
        },
        LogicFormula::Conj(x) => LogicFormula::Conj(
            x.iter().map(|a| subst(f, sub_exp, base_elem, moves, a)).collect(),
        ),
        LogicFormula::Disj(x) => LogicFormula::Disj(
            x.iter().map(|a| subst(f, sub_exp, base_elem, moves, a)).collect(),
        ),
        _ => curr_formula.clone(),
    }
}

/// output the argument of a function
fn get_args(exp: &ExpFixEq) -> Vec<ExpFixEq> {
    match exp {
        ExpFixEq::And(l, r) | ExpFixEq::Or(l, r) => {
            vec![*l.clone(), *r.clone()]
        }
        ExpFixEq::CustomOp(_, args) => args.clone(),
        id @ ExpFixEq::Id(_) => vec![id.clone()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fixpoint_system::{ExpFixEq, FixEq, FixType};
    use symbolic_exists_moves::{LogicFormula, SymbolicExistsMove};

    // fn subst(
    //     f: &Vec<FixEq>,
    //     sub_exp: &ExpFixEq,
    //     moves: &Vec<SymbolicExistsMove>,
    //     curr_formula: &LogicFormula,
    // )
    #[test]
    fn subst_basic_example() {
        let fix_eq_1 = ExpFixEq::And(
            Box::new(ExpFixEq::CustomOp("p".to_string(), vec![])), 
            Box::new(ExpFixEq::CustomOp("box".to_string(), vec![ExpFixEq::Id("x_1".to_string())]))
        );

        let formula_p_b = LogicFormula::True;
        let formula_box_b = LogicFormula::Conj(vec![
            LogicFormula::BaseElem("{d}".to_string(), 1),
            LogicFormula::BaseElem("{e}".to_string(), 1)
        ]);

        let moves = vec![
            SymbolicExistsMove {formula: formula_p_b, func_name: "p".to_string(), base_elem: "{b}".to_string()},
            SymbolicExistsMove {formula: formula_box_b, func_name: "box".to_string(), base_elem: "{b}".to_string()},
        ];

        let basis = vec!["{a}", "{b}", "{c}", "{d}", "{e}"].iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let fix_eq = vec![
            FixEq {var: "x_1".to_string(), fix_ty: FixType::Max, exp: fix_eq_1.clone()},
            //FixEq {var: "x_2".to_string(), fix_ty: FixType::Min, formula:}
        ];

        assert_eq!(
            compose_move_base(&fix_eq, &"{b}".to_string(), &fix_eq_1, &moves),
            LogicFormula::Conj(vec![
                LogicFormula::True,
                LogicFormula::Conj(vec![
                    LogicFormula::BaseElem("{d}".to_string(), 1),
                    LogicFormula::BaseElem("{e}".to_string(), 1)
                    ])
            ])
        )

    }
}
