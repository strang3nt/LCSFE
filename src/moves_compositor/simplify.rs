use crate::parser::symbolic_exists_moves::LogicFormula;

pub fn simplify(s: &LogicFormula) -> LogicFormula {
    match s {
        LogicFormula::Conj(x) => {
            let simplified = x
                .iter()
                .map(|i| simplify(i))
                .filter(|i| i != &LogicFormula::True)
                .collect::<Vec<_>>();

            if simplified.is_empty() {
                LogicFormula::True
            } else {
                if simplified.contains(&LogicFormula::False) {
                    LogicFormula::False
                } else {
                    LogicFormula::Conj(simplified)
                }
            }
        }

        LogicFormula::Disj(x) => {
            let simplified = x
                .iter()
                .map(|i| simplify(i))
                .filter(|i| i != &LogicFormula::False)
                .collect::<Vec<_>>();

            if simplified.is_empty() {
                LogicFormula::False
            } else {
                if simplified.contains(&LogicFormula::True) {
                    LogicFormula::True
                } else {
                    LogicFormula::Disj(simplified)
                }
            }
        }
        _ => s.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symplify_conj_false() {
        let formula = LogicFormula::Conj(vec![
            LogicFormula::BaseElem("a".to_string(), 1),
            LogicFormula::False,
            LogicFormula::BaseElem("b".to_string(), 3),
        ]);
        assert_eq!(simplify(&formula), LogicFormula::False);
    }

    #[test]
    fn symplify_conj_true() {
        let formula = LogicFormula::Conj(vec![
            LogicFormula::BaseElem("a".to_string(), 1),
            LogicFormula::True,
            LogicFormula::BaseElem("b".to_string(), 3),
        ]);
        assert_eq!(
            simplify(&formula),
            LogicFormula::Conj(vec![
                LogicFormula::BaseElem("a".to_string(), 1),
                LogicFormula::BaseElem("b".to_string(), 3)
            ])
        )
    }

    #[test]
    fn symplify_disj_true() {
        let formula = LogicFormula::Disj(vec![
            LogicFormula::BaseElem("a".to_string(), 1),
            LogicFormula::True,
            LogicFormula::BaseElem("b".to_string(), 3),
        ]);
        assert_eq!(simplify(&formula), LogicFormula::True);
    }

    #[test]
    fn symplify_disj_false() {
        let formula = LogicFormula::Disj(vec![
            LogicFormula::BaseElem("a".to_string(), 1),
            LogicFormula::False,
            LogicFormula::BaseElem("b".to_string(), 3),
        ]);
        assert_eq!(
            simplify(&formula),
            LogicFormula::Disj(vec![
                LogicFormula::BaseElem("a".to_string(), 1),
                LogicFormula::BaseElem("b".to_string(), 3)
            ])
        )
    }

    #[test]
    fn symplify_disj_nested_false() {
        let formula = LogicFormula::Disj(vec![
            LogicFormula::BaseElem("a".to_string(), 1),
            LogicFormula::Conj(vec![
                LogicFormula::BaseElem("a".to_string(), 1),
                LogicFormula::False,
                LogicFormula::BaseElem("b".to_string(), 3),
            ]),
            LogicFormula::BaseElem("b".to_string(), 3),
        ]);
        assert_eq!(
            simplify(&formula),
            LogicFormula::Disj(vec![
                LogicFormula::BaseElem("a".to_string(), 1),
                LogicFormula::BaseElem("b".to_string(), 3)
            ])
        )
    }

    #[test]
    fn symplify_disj_nested_true() {
        let formula = LogicFormula::Disj(vec![
            LogicFormula::BaseElem("a".to_string(), 1),
            LogicFormula::Conj(vec![
                LogicFormula::BaseElem("a".to_string(), 1),
                LogicFormula::True,
                LogicFormula::BaseElem("b".to_string(), 3),
            ]),
            LogicFormula::BaseElem("b".to_string(), 3),
        ]);
        assert_eq!(
            simplify(&formula),
            LogicFormula::Disj(vec![
                LogicFormula::BaseElem("a".to_string(), 1),
                LogicFormula::Conj(vec![
                    LogicFormula::BaseElem("a".to_string(), 1),
                    LogicFormula::BaseElem("b".to_string(), 3),
                ]),
                LogicFormula::BaseElem("b".to_string(), 3)
            ])
        )
    }
}
