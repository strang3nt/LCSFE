use std::fmt::Display;
use std::{ops::Deref, rc::Rc};

use bimap::BiMap;

use super::fixpoint_system::ExpFixEq;
use super::symbolic_exists_moves::LogicFormula as OldLogicFormula;
use super::{
    fixpoint_system::FixEq, symbolic_exists_moves::SymbolicExistsMoves as NotComposedMoves,
};

pub struct SymbolicExistsMoves {
    symbolic_moves: Vec<Rc<Node<FormulaOperator>>>,
    //    a    b   c   d   e
    // 1 n_1
    // 2
    // 3
    // 4
    // phi(a)(1)
    // symbolic_moves[0 * 4 + 0]
    basis_map: BiMap<usize, String>,
}

impl SymbolicExistsMoves {
    pub fn new(
        equations: &[FixEq],
        moves: &NotComposedMoves,
        basis: &[String],
    ) -> SymbolicExistsMoves {
        let mut symbolic_moves_composed: Vec<Rc<Node<FormulaOperator>>> =
            Vec::with_capacity(equations.len() * basis.len());
        let basis = basis.iter().cloned().enumerate().collect::<BiMap<_, _>>();
        for i in 0..equations.len() {
            for b_i in 0..basis.len() {
                let n = Self::compose_moves(equations, &equations[i].exp, moves, &basis, b_i);
                symbolic_moves_composed.push(Self::simplify(n));
            }
        }

        SymbolicExistsMoves { symbolic_moves: symbolic_moves_composed, basis_map: basis }
    }

    #[inline(always)]
    pub fn get_basis_usize(&self, b: &str) -> usize {
        *self.basis_map.get_by_right(b).unwrap()
    }

    #[inline]
    fn compose_moves(
        equations: &[FixEq],
        sub_exp: &ExpFixEq,
        moves: &NotComposedMoves,
        basis_map: &BiMap<usize, String>,
        b_i: usize,
    ) -> Rc<Node<FormulaOperator>> {
        match sub_exp {
            i @ ExpFixEq::And(_, _) | i @ ExpFixEq::Or(_, _) => {
                let n = Node {
                    val: if matches!(i, ExpFixEq::And(_, _)) {
                        FormulaOperator::And
                    } else {
                        FormulaOperator::Or
                    },
                    children: vec![
                        Self::subst(
                            equations,
                            i,
                            moves,
                            &Rc::new(Node {
                                val: FormulaOperator::Atom(BasisElem { b: b_i, i: 1 }),
                                children: vec![],
                            }),
                            basis_map,
                        ),
                        Self::subst(
                            equations,
                            i,
                            moves,
                            &Rc::new(Node {
                                val: FormulaOperator::Atom(BasisElem { b: b_i, i: 2 }),
                                children: vec![],
                            }),
                            basis_map,
                        ),
                    ],
                };
                Rc::new(n)
            }

            i @ ExpFixEq::Operator(op, _) => {
                let n = Rc::new(Self::from_formula_to_tree(
                    moves.get_formula(basis_map.get_by_left(&b_i).unwrap(), op),
                    basis_map,
                ));
                Self::subst(equations, i, moves, &n, basis_map)
            }

            ExpFixEq::Id(var) => {
                let n = Node {
                    val: FormulaOperator::Atom(BasisElem {
                        b: b_i,
                        i: Self::projection(equations, var),
                    }),
                    children: vec![],
                };
                Rc::new(n)
            }
        }
    }

    #[inline]
    fn from_formula_to_tree(
        formula: &OldLogicFormula,
        basis_map: &BiMap<usize, String>,
    ) -> Rc<Node<FormulaOperator>> {
        match formula {
            OldLogicFormula::BasisElem(b, i) => {
                let n = Node {
                    val: FormulaOperator::Atom(BasisElem {
                        b: *basis_map.get_by_right(b).unwrap(),
                        i: *i,
                    }),
                    children: vec![],
                };
                Rc::new(n)
            }
            i @ OldLogicFormula::True | i @ OldLogicFormula::False => Rc::new(Node {
                val: if matches!(i, OldLogicFormula::True) {
                    FormulaOperator::And
                } else {
                    FormulaOperator::Or
                },
                children: vec![],
            }),
            i @ OldLogicFormula::Conj(x) | i @ OldLogicFormula::Disj(x) => {
                let n = Node::<FormulaOperator> {
                    val: if matches!(i, OldLogicFormula::Conj(_)) {
                        FormulaOperator::And
                    } else {
                        FormulaOperator::Or
                    },
                    children: x.iter().map(|a| Self::from_formula_to_tree(a, basis_map)).collect(),
                };
                Rc::new(n)
            }
        }
    }

    #[inline]
    fn subst(
        equations: &[FixEq],
        sub_exp: &ExpFixEq,
        moves: &NotComposedMoves,
        curr_formula: &Rc<Node<FormulaOperator>>,
        basis_map: &BiMap<usize, String>,
    ) -> Rc<Node<FormulaOperator>> {
        match curr_formula.deref() {
            Node { val: FormulaOperator::Atom(BasisElem { b, i }), .. } => {
                let args = match sub_exp {
                    ExpFixEq::And(l, r) | ExpFixEq::Or(l, r) => {
                        vec![*l.clone(), *r.clone()]
                    }
                    ExpFixEq::Operator(_, args) => args.to_vec(),
                    ExpFixEq::Id(_) => vec![sub_exp.clone()],
                };

                Self::compose_moves(equations, &args[i - 1], moves, basis_map, *b)
            }
            Node { val: val @ FormulaOperator::And, children, .. }
            | Node { val: val @ FormulaOperator::Or, children, .. } => {
                let n = Node::<FormulaOperator> {
                    val: val.clone(),
                    children: children
                        .iter()
                        .map(|a| Self::subst(equations, sub_exp, moves, a, basis_map))
                        .collect(),
                };
                Rc::new(n)
            }
        }
    }

    #[inline(always)]
    fn projection(f: &[FixEq], curr_var: &String) -> usize {
        f.iter().position(|FixEq { var, .. }| var == curr_var).unwrap() + 1
    }

    #[inline]
    pub fn simplify(f: Rc<Node<FormulaOperator>>) -> Rc<Node<FormulaOperator>> {
        match f.deref() {
            Node { val: val @ FormulaOperator::And, children }
            | Node { val: val @ FormulaOperator::Or, children }
                if !children.is_empty() =>
            {
                let simplified_children: Vec<_> = children
                    .iter()
                    .filter_map(|x| {
                        let n = Self::simplify(x.clone());
                        if (Self::is_formula_false(&n) && *val == FormulaOperator::Or)
                            || (Self::is_formula_true(&n) && *val == FormulaOperator::And)
                        {
                            None
                        } else {
                            Some(n)
                        }
                    })
                    .collect();

                if simplified_children.len() == 1 {
                    simplified_children[0].clone()
                } else if simplified_children.iter().any(|x| Self::is_formula_false(x))
                    && val == &FormulaOperator::And
                {
                    Rc::new(Node { val: FormulaOperator::Or, children: vec![] })
                } else if simplified_children.iter().any(|x| Self::is_formula_true(x))
                    && val == &FormulaOperator::Or
                {
                    Rc::new(Node { val: FormulaOperator::And, children: vec![] })
                } else {
                    Rc::new(Node { val: val.clone(), children: simplified_children })
                }
            }
            _ => f,
        }
    }

    #[inline(always)]
    fn is_formula_true(f: &Node<FormulaOperator>) -> bool {
        f.val == FormulaOperator::And && f.children.is_empty()
    }

    #[inline(always)]
    pub fn is_formula_false(f: &Node<FormulaOperator>) -> bool {
        f.val == FormulaOperator::Or && f.children.is_empty()
    }

    #[inline(always)]
    pub fn get_formula(&self, b: usize, i: usize) -> Rc<Node<FormulaOperator>> {
        self.symbolic_moves[i * self.basis_map.len() + b].clone()
    }

    #[inline(always)]
    fn print_logic_formula(&self, n: &Rc<Node<FormulaOperator>>) -> String {
        match n.deref() {
            Node { val, children, .. } if !children.is_empty() => {
                let children: Vec<String> =
                    children.iter().map(|x| self.print_logic_formula(x)).collect();
                children.join(if matches!(val, FormulaOperator::And) { " and " } else { " or " })
            }
            Node { val: FormulaOperator::Atom(BasisElem { b, i }), .. } => {
                format!("[{}, {}]", self.basis_map.get_by_left(b).unwrap(), i)
            }
            Node { val, .. } => {
                if matches!(val, FormulaOperator::And) {
                    "true".to_owned()
                } else {
                    "false".to_owned()
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct BasisElem {
    pub b: usize,
    pub i: usize,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum FormulaOperator {
    And,
    Or,
    Atom(BasisElem),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Node<T> {
    pub val: T,
    pub children: Vec<Rc<Node<T>>>,
}

impl Display for SymbolicExistsMoves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l: Vec<_> = self
            .symbolic_moves
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let b_i = self.basis_map.get_by_left(&(i % self.basis_map.len())).unwrap();
                format!("phi({})({}) = {}", b_i, i, self.print_logic_formula(x))
            })
            .collect();
        write!(f, "{}", l.join(";\n"))
    }
}
