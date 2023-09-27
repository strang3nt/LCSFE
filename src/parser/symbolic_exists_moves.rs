#[derive(Debug, Eq, PartialEq)]
pub struct SymbolicSystem(pub Vec<SymbolicExistsMove>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SymbolicExistsMove {
    pub formula: LogicFormula, 
    pub func_name: usize, // TODO this type is only temporary
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
