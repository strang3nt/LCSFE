mod play_data;
pub mod player;
pub mod position;
mod position_counter_set;

use std::collections::BTreeSet;
use std::collections::HashSet;
use std::time::Instant;

use crate::ast::fixpoint_system::{FixEq, FixType};
use crate::ast::symbolic_exists_moves::{
    LogicFormula, SymbolicExistsMoveComposed,
};
use itertools::Itertools;
use play_data::PlayData;
use player::Player;
use position::{AdamPos, EvePos, Position};
use position_counter_set::{Justification, PositionCounterSet};

type Playlist = Vec<(PlayData, HashSet<PlayData>)>;
type Counter = Vec<u32>;

pub struct ParityGame<'a> {
    pub fix_system: &'a Vec<FixEq>,
    pub symbolic_moves: &'a Vec<SymbolicExistsMoveComposed>,
    pub basis: &'a Vec<String>,
}

impl<'a> ParityGame<'a> {
    pub fn local_check(&self, c: Position) -> Player {
        println!(
            "The parameters are:\n\n{}\n\n{}\n\n{:?}\n",
            self.fix_system
                .iter()
                .map(|x| format!("{};", x))
                .collect::<Vec<String>>()
                .join("\n"),
            self.symbolic_moves
                .iter()
                .map(|x| format!("{};", x))
                .collect::<Vec<String>>()
                .join("\n"),
            self.basis
        );

        let m: usize = self.fix_system.len();

        self.explore(
            PlayData { pos: c, k: vec![0; m + 1] },
            &mut vec![],
            &mut PositionCounterSet::new(),
            &mut PositionCounterSet::new(),
        )
    }

    fn explore(
        &self,
        play_data: PlayData,
        pl: &mut Playlist,
        assumptions: &mut PositionCounterSet<Instant>,
        decisions: &mut PositionCounterSet<(Justification, Instant)>,
    ) -> Player {
        let c = &play_data.pos;
        let k = &play_data.k;

        if self.is_empty(c) {
            let opponent = Player::get_opponent(&Position::get_controller(&c));
            decisions.get_mut_p(&opponent).insert(
                play_data.clone(),
                (Justification::Truth, Instant::now()),
            );
            self.backtrack(&opponent, c, pl, assumptions, decisions)
        } else if let Some(p) = self.contains(&decisions, &play_data) {
            self.backtrack(&p, c, pl, assumptions, decisions)
        } else if let Some((PlayData { pos: _, k: kp }, _)) =
            pl.iter().find(|(PlayData { pos: cp, k: _ }, _)| cp == c)
        {
            let p = match self.le(kp, k, &Player::Eve) {
                true => Player::Eve,
                false => Player::Adam,
            };
            assumptions.get_mut_p(&p).insert(
                PlayData { pos: c.clone(), k: kp.clone() },
                Instant::now(),
            );
            self.backtrack(&p, &c, pl, assumptions, decisions)
        } else {
            let kp = Self::next(k, Position::priority(c));

            let mut pi = match c {
                Position::Adam(pos) => self
                    .universal_move(&pos)
                    .into_iter()
                    .map(|b_i| PlayData {
                        pos: Position::Eve(b_i),
                        k: kp.clone(),
                    })
                    .collect::<HashSet<_>>(),

                Position::Eve(b_i) => {
                    let formula = Self::get_formula(self.symbolic_moves, b_i);
                    self.existential_move(formula)
                        .unwrap()
                        .into_iter()
                        .map(|x| PlayData {
                            pos: Position::Adam(x),
                            k: kp.clone(),
                        })
                        .collect::<HashSet<_>>()
                }
            };

            let cp = pi.iter().next().unwrap().clone();
            println!("{:?}", pi);
            pi.remove(&cp);
            println!("{:?}", &cp);
            pl.push((play_data, pi));
            self.explore(cp, pl, assumptions, decisions)
        }
    }

    fn backtrack(
        &self,
        p: &Player,
        c: &Position,
        pl: &mut Playlist,
        assumptions: &mut PositionCounterSet<Instant>,
        decisions: &mut PositionCounterSet<(Justification, Instant)>,
    ) -> Player {
        if let Some((play_data, mut pi)) = pl.pop() {
            let cp = &play_data.pos;
            let kp = &play_data.k;

            if &Position::get_controller(&cp) != p && !pi.is_empty() {
                let play_data_p = pi.iter().next().unwrap().clone();
                pi.remove(&play_data_p);
                pl.push((play_data, pi));
                self.explore(play_data_p, pl, assumptions, decisions)
            } else {
                if Position::get_controller(&cp) == *p {
                    decisions.get_mut_p(p).insert(
                        play_data.clone(),
                        (
                            Justification::SetOfMoves(HashSet::from([
                                c.clone()
                            ])),
                            Instant::now(),
                        ),
                    );
                } else {
                    match &cp {
                        Position::Eve(b_i) => {
                            let formula =
                                Self::get_formula(self.symbolic_moves, b_i);
                            decisions.get_mut_p(&Player::Eve).insert(
                                PlayData { pos: cp.clone(), k: kp.clone() },
                                (
                                    Justification::SetOfMoves(
                                        self.existential_move(formula)
                                            .unwrap_or_default()
                                            .into_iter()
                                            .map(|x| Position::Adam(x))
                                            .collect(),
                                    ),
                                    Instant::now(),
                                ),
                            );
                        }
                        Position::Adam(x) => {
                            decisions.get_mut_p(&Player::Adam).insert(
                                PlayData { pos: cp.clone(), k: kp.clone() },
                                (
                                    Justification::SetOfMoves(
                                        self.universal_move(x)
                                            .into_iter()
                                            .map(|b_i| Position::Eve(b_i))
                                            .collect(),
                                    ),
                                    Instant::now(),
                                ),
                            );
                        }
                    }
                }
                assumptions.get_mut_p(p).remove(&play_data);
                let opponent = Player::get_opponent(p);
                if let Some(_) = assumptions.get_p(&opponent).get(&play_data) {
                    Self::forget(&opponent, &play_data, assumptions, decisions);
                    assumptions.get_mut_p(&opponent).remove(&play_data);
                };

                self.backtrack(&p, &cp, pl, assumptions, decisions)
            }
        } else {
            p.clone()
        }
    }

    fn forget(
        p: &Player,
        c: &PlayData,
        assumptions: &PositionCounterSet<Instant>,
        decisions: &mut PositionCounterSet<(Justification, Instant)>,
    ) {
        if let Some(after_not_valid) = &mut assumptions.get_p(p).get(c).cloned()
        {
            decisions
                .get_mut_p(p)
                .retain(|_, (_, inst)| inst < after_not_valid);
        }
    }

    /// TODO: check return type: if should be reference or copy
    pub fn get_formula(
        s: &'a Vec<SymbolicExistsMoveComposed>,
        c: &EvePos,
    ) -> &'a LogicFormula {
        let EvePos { b, i } = c;
        if let Some(f) = s.iter().find(
            |SymbolicExistsMoveComposed {
                 formula: _,
                 basis_elem: base_elem,
                 func_name,
             }| { base_elem == b && func_name == i },
        ) {
            &f.formula
        } else {
            panic!(
                "No symbolic exists move for basis element {} at position {}",
                b, i
            )
        }
    }

    fn is_empty(&self, p: &Position) -> bool {
        match p {
            Position::Eve(b_i) => {
                Self::get_formula(self.symbolic_moves, b_i)
                    == &LogicFormula::False
            }
            Position::Adam(x) => HashSet::is_empty(&self.universal_move(x)),
        }
    }

    /// Moves are supposed to be normalized
    fn existential_move(&self, f: &LogicFormula) -> Option<HashSet<AdamPos>> {
        let mut c: HashSet<AdamPos> = HashSet::new();

        match f {
            LogicFormula::False => None,
            LogicFormula::True => Some(c),
            LogicFormula::BasisElem(b, i) => {
                let mut x =
                    vec![BTreeSet::<String>::new(); self.fix_system.len()];
                x[*i - 1].insert(b.clone());
                c.insert(AdamPos { x });
                Some(c)
            }
            LogicFormula::Conj(fs) => Some(
                fs.into_iter()
                    .map(|i| {
                        self.existential_move(i)
                            .unwrap_or_default()
                            .into_iter()
                            .collect::<Vec<_>>()
                    })
                    .multi_cartesian_product()
                    .map(|y| {
                        y.into_iter().fold(
                            vec![BTreeSet::new(); self.fix_system.len()],
                            |acc, AdamPos { x: elem }| {
                                acc.into_iter()
                                    .enumerate()
                                    .map(|(i, e): (usize, BTreeSet<String>)| {
                                        e.union(&elem[i])
                                            .map(|x| x.clone())
                                            .collect::<BTreeSet<String>>()
                                    })
                                    .collect::<Vec<_>>()
                            },
                        )
                    })
                    .map(|x| AdamPos { x })
                    .collect::<HashSet<_>>(),
            ), // unwrap or else none,
            LogicFormula::Disj(fs) => {
                Some(
                    fs.iter()
                        .map(|i| self.existential_move(i).unwrap_or_default()) // unwrap or else empty set
                        .flatten()
                        .collect::<HashSet<_>>(),
                )
            }
        }
    }

    fn universal_move(&self, univ_position: &AdamPos) -> HashSet<EvePos> {
        univ_position.x.iter().enumerate().fold(
            HashSet::new(),
            |mut acc, (i, x_i)| {
                for b in x_i {
                    acc.insert(EvePos { b: b.clone(), i: i + 1 });
                }
                acc
            },
        )
    }

    pub fn contains(
        &self,
        decisions: &PositionCounterSet<(Justification, Instant)>,
        play_data: &PlayData,
    ) -> Option<Player> {
        let PlayData { pos: c, k } = play_data;

        if decisions.get_p(&Player::Eve).iter().any(
            |(PlayData { pos: cp, k: kp }, _)| {
                self.leq(kp, k, &Player::Eve) && cp == c
            },
        ) {
            Some(Player::Eve)
        } else if decisions.get_p(&Player::Adam).iter().any(
            |(PlayData { pos: cp, k: kp }, _)| {
                self.leq(kp, k, &Player::Adam) && cp == c
            },
        ) {
            Some(Player::Adam)
        } else {
            None
        }
    }

    /// TODO eliminate recursion
    /// A pre-condition of this function is that the argument `f: &LogicFormula` does
    /// not have `LogicFormula::True` or `Logic::Formula::False` leaves.
    fn _build_next_move(&self, f: &LogicFormula) -> Vec<BTreeSet<String>> {
        let mut c: Vec<BTreeSet<String>> =
            vec![BTreeSet::new(); self.fix_system.len()];

        match f {
            LogicFormula::BasisElem(b, i) => {
                c[i.clone()].insert(b.clone());
            }
            LogicFormula::Conj(fs) => {
                for f in fs {
                    let mut j = 0;
                    for fj in self._build_next_move(f) {
                        c[j].extend(fj);
                        j = j + 1;
                    }
                }
            }
            LogicFormula::Disj(fs) => c = self._build_next_move(&fs[0]),
            _ => panic!("Formula {:?} has true or false leaves", f),
        };
        c
    }

    /// Precondition: the formula `f` is simplified, using the function `reduce(f)`,
    /// and `nextMove(f)` has never been called before.
    /// TODO: values of logic formula true or false should return different values.
    fn _next_move(&self, f: &LogicFormula) -> Option<Vec<BTreeSet<String>>> {
        match f {
            LogicFormula::False => None,
            LogicFormula::True => Some(vec![]),
            _ => Some(self._build_next_move(f)),
        }
    }

    pub fn le(&self, k: &[u32], kp: &[u32], p: &Player) -> bool {
        let mut n = 0;
        let m = k.len();
        for i in 1..m {
            if k[i] != kp[i] {
                n = i;
            };
        }

        if n == 0 {
            false
        } else {
            let result_for_eve = match self.fix_system[n - 1].fix_ty {
                FixType::Max => k[n] < kp[n],
                FixType::Min => k[n] > kp[n],
            };

            match p {
                Player::Adam => !result_for_eve,
                Player::Eve => result_for_eve,
            }
        }
    }

    /// Implements a total order for the counter:
    ///
    ///  - it is the case that $k <_\exists k'$, whenever
    ///    the largest $i$ such that $k_i\neq k'_i$ is the
    ///    index of a greatest fixpoint, that is to say whenever
    ///    `self.fix_types[i]` has value `Max`, or if the value is
    ///    `Min` and $k_i > k'_i$
    ///  - we say $k <_\forall k'$ whenever it is not true that
    ///    $k' <_\exists k$
    ///  - $k \leq_P k'$ whenever $k <_P k$ or $k = k$.
    ///
    /// > Notation: $k$, $k'$ are vectors, and $P\in \{\exists, \forall\}$.
    pub fn leq(&self, k: &[u32], kp: &[u32], p: &Player) -> bool {
        k == kp || self.le(k, kp, p)
    }

    /// Updates the counter, such that each non-zero priority (argument `i`) is associated
    /// with the number of times the priority has been encountered in the play
    /// since a higher priority was last faced. Note that:
    ///
    ///  - $next(k, 0) = k$
    ///  -  $next(k, i) = k'$.
    ///
    fn next(k: &Counter, i: usize) -> Counter {
        let mut kp = vec![0; k.len()];
        if i == 0 {
            k.clone()
        } else {
            kp[i] = k[i] + 1;
            for j in i + 1..kp.len() {
                kp[j] = k[j]
            }
            kp
        }
    }
}
