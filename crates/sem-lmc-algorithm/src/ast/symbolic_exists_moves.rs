#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SymbolicExistsMoveComposed {
    pub formula: LogicFormula,
    pub func_name: usize,
    pub basis_elem: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SymbolicExistsMove {
    pub formula: LogicFormula,
    pub func_name: String,
    pub basis_elem: String,
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
                format!("{}", to_vec_of_str(xs).join(" and "))
            }
            LogicFormula::Disj(xs) => {
                format!("{}", to_vec_of_str(xs).join(" or "))
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
