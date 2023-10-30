use crate::ast::symbolic_exists_moves::LogicFormula;

pub fn simplify(s: LogicFormula) -> LogicFormula {
    match s {
        LogicFormula::Conj(x) => {
            let simplified = x
                .into_iter()
                .filter_map(|i| {let i = simplify(i); if i != LogicFormula::True { Some (i)} else { None }})
                .collect::<Vec<_>>();

            if simplified.is_empty() {
                LogicFormula::True
            } else if simplified.contains(&LogicFormula::False) {
                    LogicFormula::False
            } else {
                extract(LogicFormula::Conj(simplified))
            }
            
        }

        LogicFormula::Disj(x) => {
            let simplified = x
                .into_iter()
                .filter_map(|i| {let i = simplify(i); if i != LogicFormula::False { Some(i) } else { None }})
                .collect::<Vec<_>>();

            if simplified.is_empty() {
                LogicFormula::False
            } else if simplified.contains(&LogicFormula::True) {
                LogicFormula::True
            } else {
                extract(LogicFormula::Disj(simplified))
            }
        }
        _ => s,
    }
}

#[inline]
fn extract(f: LogicFormula) -> LogicFormula {
    match f {
        LogicFormula::Conj(x) if x.len() == 1 => x[0].clone(),
        LogicFormula::Disj(x) if x.len() == 1 => x[0].clone(),
        _ => f,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symplify_conj_false() {
        let formula = LogicFormula::Conj(vec![
            LogicFormula::BasisElem("a".to_string(), 1),
            LogicFormula::False,
            LogicFormula::BasisElem("b".to_string(), 3),
        ]);
        assert_eq!(simplify(formula), LogicFormula::False);
    }

    #[test]
    fn symplify_conj_true() {
        let formula = LogicFormula::Conj(vec![
            LogicFormula::BasisElem("a".to_string(), 1),
            LogicFormula::True,
            LogicFormula::BasisElem("b".to_string(), 3),
        ]);
        assert_eq!(
            simplify(formula),
            LogicFormula::Conj(vec![
                LogicFormula::BasisElem("a".to_string(), 1),
                LogicFormula::BasisElem("b".to_string(), 3)
            ])
        )
    }

    #[test]
    fn symplify_disj_true() {
        let formula = LogicFormula::Disj(vec![
            LogicFormula::BasisElem("a".to_string(), 1),
            LogicFormula::True,
            LogicFormula::BasisElem("b".to_string(), 3),
        ]);
        assert_eq!(simplify(formula), LogicFormula::True);
    }

    #[test]
    fn symplify_disj_false() {
        let formula = LogicFormula::Disj(vec![
            LogicFormula::BasisElem("a".to_string(), 1),
            LogicFormula::False,
            LogicFormula::BasisElem("b".to_string(), 3),
        ]);
        assert_eq!(
            simplify(formula),
            LogicFormula::Disj(vec![
                LogicFormula::BasisElem("a".to_string(), 1),
                LogicFormula::BasisElem("b".to_string(), 3)
            ])
        )
    }

    #[test]
    fn symplify_disj_nested_false() {
        let formula = LogicFormula::Disj(vec![
            LogicFormula::BasisElem("a".to_string(), 1),
            LogicFormula::Conj(vec![
                LogicFormula::BasisElem("a".to_string(), 1),
                LogicFormula::False,
                LogicFormula::BasisElem("b".to_string(), 3),
            ]),
            LogicFormula::BasisElem("b".to_string(), 3),
        ]);
        assert_eq!(
            simplify(formula),
            LogicFormula::Disj(vec![
                LogicFormula::BasisElem("a".to_string(), 1),
                LogicFormula::BasisElem("b".to_string(), 3)
            ])
        )
    }

    #[test]
    fn symplify_disj_nested_true() {
        let formula = LogicFormula::Disj(vec![
            LogicFormula::BasisElem("a".to_string(), 1),
            LogicFormula::Conj(vec![
                LogicFormula::BasisElem("a".to_string(), 1),
                LogicFormula::True,
                LogicFormula::BasisElem("b".to_string(), 3),
            ]),
            LogicFormula::BasisElem("b".to_string(), 3),
        ]);
        assert_eq!(
            simplify(formula),
            LogicFormula::Disj(vec![
                LogicFormula::BasisElem("a".to_string(), 1),
                LogicFormula::Conj(vec![
                    LogicFormula::BasisElem("a".to_string(), 1),
                    LogicFormula::BasisElem("b".to_string(), 3),
                ]),
                LogicFormula::BasisElem("b".to_string(), 3)
            ])
        )
    }

    #[test]
    fn simplify_extract() {
        assert_eq!(
            simplify(LogicFormula::Conj(vec![
                LogicFormula::True,
                LogicFormula::Conj(vec![
                    LogicFormula::BasisElem("{d}".to_string(), 1),
                    LogicFormula::BasisElem("{e}".to_string(), 1)
                ])
            ])),
            LogicFormula::Conj(vec![
                LogicFormula::BasisElem("{d}".to_string(), 1),
                LogicFormula::BasisElem("{e}".to_string(), 1)
            ])
        )
    }
}
