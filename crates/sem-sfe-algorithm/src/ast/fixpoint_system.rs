use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FixType {
    Min,
    Max,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixEq {
    pub var: String,
    pub fix_ty: FixType,
    pub exp: ExpFixEq,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExpFixEq {
    And(Box<ExpFixEq>, Box<ExpFixEq>),
    Or(Box<ExpFixEq>, Box<ExpFixEq>),
    Operator(String, Vec<ExpFixEq>),
    Id(String),
}

impl fmt::Display for FixType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fix_ty = match self {
            FixType::Max => "=max",
            FixType::Min => "=min",
        };
        write!(f, "{}", fix_ty)
    }
}

impl fmt::Display for ExpFixEq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sub_arg = |x: &ExpFixEq| match x {
            ExpFixEq::Id(s) => s.to_owned(),
            ExpFixEq::Operator(_, _) => format!("{}", x),
            _ => format!("({})", x),
        };

        let exp_fix_eq: String = match self {
            ExpFixEq::And(l, r) => {
                format!("{} and {}", sub_arg(l), sub_arg(r))
            }
            ExpFixEq::Or(l, r) => format!("{} or {}", sub_arg(l), sub_arg(r)),
            ExpFixEq::Operator(op, args) => {
                let args = args
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>();
                format!("{}({})", op.to_owned(), args.join(", "))
            }
            ExpFixEq::Id(s) => s.to_owned(),
        };
        write!(f, "{}", exp_fix_eq)
    }
}

impl fmt::Display for FixEq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.var, self.fix_ty, self.exp)
    }
}
