#[derive(Debug, Clone)]
pub struct FixpointSystem(pub Vec<Eq>);

#[derive(Debug, Clone)]
pub enum Eq {
    Min(String, ExpEq),
    Max(String, ExpEq),
}

#[derive(Debug, Clone)]
pub enum ExpEq {
    And(Box<ExpEq>, Box<ExpEq>),
    Or(Box<ExpEq>, Box<ExpEq>),
    CustomOp(String, Vec<ExpEq>),
    Id(String),
}
