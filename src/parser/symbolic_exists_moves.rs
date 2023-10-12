#[derive(Debug, Clone, Eq, PartialEq)]

pub struct SymbolicExistsMoveComposed {
    pub formula: LogicFormula,
    pub func_name: usize,
    pub base_elem: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SymbolicExistsMove {
    pub formula: LogicFormula,
    pub func_name: String,
    pub base_elem: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LogicFormula {
    BaseElem(String, usize),
    True,
    False,
    Conj(Vec<LogicFormula>),
    Disj(Vec<LogicFormula>),
}
