#[derive(Debug, Clone)]
pub enum FixType {
    Min,
    Max,
}

#[derive(Debug, Clone)]
pub struct FixEq {
    pub var: String,
    pub fix_ty: FixType,
    pub exp: ExpFixEq,
}

#[derive(Debug, Clone)]
pub enum ExpFixEq {
    And(Box<ExpFixEq>, Box<ExpFixEq>),
    Or(Box<ExpFixEq>, Box<ExpFixEq>),
    Operator(String, Vec<ExpFixEq>),
    Id(String),
}
