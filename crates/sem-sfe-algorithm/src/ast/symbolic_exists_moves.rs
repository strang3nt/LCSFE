use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SymbolicExistsMoveComposed {
    pub formula: LogicFormula,
    pub func_name: usize,
    pub basis_elem: String,
}

#[derive(Debug)]
pub struct SymbolicExistsMoves {
    pub basis_map: HashMap<String, usize>,
    pub fun_map: HashMap<String, usize>,
    pub formulas: Vec<LogicFormula>,
}

impl SymbolicExistsMoves {
    pub fn get_formula(
        &self,
        basis_elem: &String,
        fun: &String,
    ) -> &LogicFormula {
        &self.formulas[self.fun_map.get(fun).unwrap() * self.basis_map.len()
            + self.basis_map.get(basis_elem).unwrap()]
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LogicFormula {
    BasisElem(String, usize),
    True,
    False,
    Conj(Vec<LogicFormula>),
    Disj(Vec<LogicFormula>),
}

impl std::fmt::Display for LogicFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_vec_of_str = |xs: &Vec<LogicFormula>| {
            xs.iter().map(|x| format!("{}", x)).collect::<Vec<String>>()
        };

        let formula = match self {
            LogicFormula::BasisElem(b, i) => format!("[{}, {}]", b, i),
            LogicFormula::Conj(xs) => {
                to_vec_of_str(xs).join(" and ").to_string()
            }
            LogicFormula::Disj(xs) => {
                to_vec_of_str(xs).join(" or ").to_string()
            }
            LogicFormula::False => "false".to_owned(),
            LogicFormula::True => "true".to_owned(),
        };

        write!(f, "{}", formula)
    }
}

impl std::fmt::Display for SymbolicExistsMoveComposed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "phi({})({}) = {}",
            self.basis_elem, self.func_name, self.formula
        )
    }
}
