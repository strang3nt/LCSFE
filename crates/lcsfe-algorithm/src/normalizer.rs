use crate::ast::fixpoint_system::{ExpFixEq, FixEq, FixType};
use rustc_hash::FxHashMap as HashMap;

pub fn normalize_system(fix_system: Vec<FixEq>) -> (Vec<FixEq>, HashMap<String, String>) {
    let mut var_map = fix_system
        .iter()
        .enumerate()
        .map(|(i, FixEq { var, .. })| (var.to_owned(), format!("x_{}", i + 1)))
        .collect::<HashMap<String, String>>();

    let normalized_system = fix_system
        .into_iter()
        .flat_map(|fix_eq| normalize_equation(fix_eq, &mut var_map))
        .collect::<Vec<_>>();

    (normalized_system, var_map)
}

fn normalize_equation(
    FixEq { var, fix_ty, exp }: FixEq,
    var_map: &mut HashMap<String, String>,
) -> Vec<FixEq> {
    match exp {
        ExpFixEq::Id(x) => vec![FixEq {
            var: var_map.get(&var).cloned().unwrap(),
            fix_ty,
            exp: var_map.get(&x).cloned().map(ExpFixEq::Id).unwrap(),
        }],

        ExpFixEq::Operator(name, args) => {
            let normalized_args = normalize_args(fix_ty.clone(), args, var_map);

            let mut op_normalized = vec![FixEq {
                var: var_map.get(&var).cloned().unwrap(),
                fix_ty,
                exp: ExpFixEq::Operator(
                    name,
                    normalized_args
                        .iter()
                        .map(|x| var_map.get(&x[0].var).cloned().map(ExpFixEq::Id).unwrap())
                        .collect::<Vec<_>>(),
                ),
            }];
            op_normalized.extend(flatten_and_remove_identity(normalized_args));
            op_normalized
        }
        ExpFixEq::And(l, r) => {
            let normalized_args = normalize_args(fix_ty.clone(), vec![*l, *r], var_map);
            let mut and_normalized = vec![FixEq {
                var: var_map.get(&var).cloned().unwrap(),
                fix_ty,
                exp: ExpFixEq::And(
                    Box::new(
                        var_map
                            .get(&normalized_args[0][0].var)
                            .cloned()
                            .map(ExpFixEq::Id)
                            .unwrap(),
                    ),
                    Box::new(
                        var_map
                            .get(&normalized_args[1][0].var)
                            .cloned()
                            .map(ExpFixEq::Id)
                            .unwrap(),
                    ),
                ),
            }];
            and_normalized.extend(flatten_and_remove_identity(normalized_args));
            and_normalized
        }
        ExpFixEq::Or(l, r) => {
            let normalized_args = normalize_args(fix_ty.clone(), vec![*l, *r], var_map);
            let mut or_normalized = vec![FixEq {
                var: var_map.get(&var).cloned().unwrap(),
                fix_ty,
                exp: ExpFixEq::Or(
                    Box::new(
                        var_map
                            .get(&normalized_args[0][0].var)
                            .cloned()
                            .map(ExpFixEq::Id)
                            .unwrap(),
                    ),
                    Box::new(
                        var_map
                            .get(&normalized_args[1][0].var)
                            .cloned()
                            .map(ExpFixEq::Id)
                            .unwrap(),
                    ),
                ),
            }];
            or_normalized.extend(flatten_and_remove_identity(normalized_args));
            or_normalized
        }
    }
}

fn normalize_args(
    fix_ty: FixType,
    args: Vec<ExpFixEq>,
    var_map: &mut HashMap<String, String>,
) -> Vec<Vec<FixEq>> {
    args.into_iter()
        .map(|arg| match &arg {
            ExpFixEq::Id(x) => vec![FixEq {
                var: x.to_owned(),
                fix_ty: fix_ty.clone(),
                exp: arg,
            }],
            _ => {
                let new_var = format!("x_{}", var_map.len() + 1);
                var_map.insert(new_var.clone(), new_var.clone());
                normalize_equation(
                    FixEq {
                        var: new_var,
                        fix_ty: fix_ty.clone(),
                        exp: arg,
                    },
                    var_map,
                )
            }
        })
        .collect::<Vec<_>>()
}

fn flatten_and_remove_identity(new_fix_eq: Vec<Vec<FixEq>>) -> impl Iterator<Item = FixEq> {
    new_fix_eq
        .into_iter()
        .flatten()
        .filter(|x| !matches!(x, FixEq { var: x, exp: ExpFixEq::Id(y), .. } if x == y))
}

#[cfg(test)]
mod tests {

    use crate::{
        ast::fixpoint_system::{ExpFixEq, FixEq, FixType},
        normalizer,
    };

    /// νx1.µx2.(x1 ∨ x2 ∨ νx3.µx4.νx5.(x3 ∧ diamond(x4 ∧ x5)))
    /// x_1 =max x_2
    /// x_2 =min (x_1 or x_2) or x_3
    /// x_3 =max x_4
    /// x_4 =min x_5
    /// x_5 =max x_3 and diamond(x_4 and x_5)
    ///
    /// which normalized becomes the following system of equations
    ///
    /// x_1 =max x_2
    /// x_2 =min x_6 or x_3
    /// x_6 =min x_1 or x_2
    /// x_3 =max x_4
    /// x_4 =min x_5
    /// x_5 =max x_3 and x_7
    /// x_7 =max diamond(x_8)
    /// x_8 =max x_4 and x_5
    ///
    #[test]
    fn normalize_system_mu_calc() {
        let system = vec![
            FixEq {
                var: "x_1".to_owned(),
                fix_ty: FixType::Max,
                exp: ExpFixEq::Id("x_2".to_owned()),
            },
            FixEq {
                var: "x_2".to_owned(),
                fix_ty: FixType::Min,
                exp: ExpFixEq::Or(
                    Box::new(ExpFixEq::Or(
                        Box::new(ExpFixEq::Id("x_1".to_owned())),
                        Box::new(ExpFixEq::Id("x_2".to_owned())),
                    )),
                    Box::new(ExpFixEq::Id("x_3".to_owned())),
                ),
            },
            FixEq {
                var: "x_3".to_owned(),
                fix_ty: FixType::Max,
                exp: ExpFixEq::Id("x_4".to_owned()),
            },
            FixEq {
                var: "x_4".to_owned(),
                fix_ty: FixType::Min,
                exp: ExpFixEq::Id("x_5".to_owned()),
            },
            FixEq {
                var: "x_5".to_owned(),
                fix_ty: FixType::Max,
                exp: ExpFixEq::And(
                    Box::new(ExpFixEq::Id("x_3".to_string())),
                    Box::new(ExpFixEq::Operator(
                        "diamond".to_owned(),
                        vec![ExpFixEq::And(
                            Box::new(ExpFixEq::Id("x_4".to_owned())),
                            Box::new(ExpFixEq::Id("x_5".to_owned())),
                        )],
                    )),
                ),
            },
        ];

        let normalized_system = vec![
            FixEq {
                var: "x_1".to_owned(),
                fix_ty: FixType::Max,
                exp: ExpFixEq::Id("x_2".to_owned()),
            },
            FixEq {
                var: "x_2".to_owned(),
                fix_ty: FixType::Min,
                exp: ExpFixEq::Or(
                    Box::new(ExpFixEq::Id("x_6".to_owned())),
                    Box::new(ExpFixEq::Id("x_3".to_owned())),
                ),
            },
            FixEq {
                var: "x_6".to_owned(),
                fix_ty: FixType::Min,
                exp: ExpFixEq::Or(
                    Box::new(ExpFixEq::Id("x_1".to_owned())),
                    Box::new(ExpFixEq::Id("x_2".to_owned())),
                ),
            },
            FixEq {
                var: "x_3".to_owned(),
                fix_ty: FixType::Max,
                exp: ExpFixEq::Id("x_4".to_owned()),
            },
            FixEq {
                var: "x_4".to_owned(),
                fix_ty: FixType::Min,
                exp: ExpFixEq::Id("x_5".to_owned()),
            },
            FixEq {
                var: "x_5".to_owned(),
                fix_ty: FixType::Max,
                exp: ExpFixEq::And(
                    Box::new(ExpFixEq::Id("x_3".to_owned())),
                    Box::new(ExpFixEq::Id("x_7".to_owned())),
                ),
            },
            FixEq {
                var: "x_7".to_owned(),
                fix_ty: FixType::Max,
                exp: ExpFixEq::Operator("diamond".to_owned(), vec![ExpFixEq::Id("x_8".to_owned())]),
            },
            FixEq {
                var: "x_8".to_owned(),
                fix_ty: FixType::Max,
                exp: ExpFixEq::And(
                    Box::new(ExpFixEq::Id("x_4".to_owned())),
                    Box::new(ExpFixEq::Id("x_5".to_owned())),
                ),
            },
        ];

        let (fun_normalized_system, _) = normalizer::normalize_system(system);
        assert_eq!(fun_normalized_system, normalized_system)
    }
}
