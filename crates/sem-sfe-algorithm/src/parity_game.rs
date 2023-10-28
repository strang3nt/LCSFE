mod play_data;
pub mod player;
pub mod position;
mod position_counter_set;

use std::collections::BTreeSet;
use std::collections::HashSet;
use std::rc::Rc;
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

type Playlist =
    Vec<(PlayData, (Box<dyn Iterator<Item = Position>>, Rc<Counter>))>;
type Counter = Vec<u32>;

pub struct LocalAlgorithm<'a> {
    pub fix_system: &'a Vec<FixEq>,
    pub symbolic_moves: &'a Vec<SymbolicExistsMoveComposed>,
    pub basis: &'a Vec<String>,
}

impl<'a> LocalAlgorithm<'a> {
    pub fn local_check(&self, c: Position) -> Player {
        let m: usize = self.fix_system.len();
        self.explore(
            PlayData { pos: c, k: Rc::new(vec![0; m]) },
            vec![],
            PositionCounterSet::new(),
            PositionCounterSet::new(),
        )
    }

    fn explore(
        &self,
        play_data: PlayData,
        mut pl: Playlist,
        mut assumptions: PositionCounterSet<Instant>,
        mut decisions: PositionCounterSet<(Justification, Instant)>,
    ) -> Player {
        let mut iter = match play_data.pos.clone() {
            Position::Eve(x) => Self::exists_move(
                self.get_formula(&x).clone(),
                self.fix_system.len(),
            )
            .peekable(),
            Position::Adam(x) => Self::universal_move(x).peekable(),
        };

        if let None = iter.peek() {
            let opponent =
                Player::get_opponent(&Position::get_controller(&play_data.pos));
            decisions.get_mut_p(&opponent).insert(
                play_data.clone(),
                (Justification::Truth, Instant::now()),
            );
            self.backtrack(opponent, play_data.pos, pl, assumptions, decisions)
        } else if let Some(p) = self.contains(&decisions, &play_data) {
            self.backtrack(p, play_data.pos, pl, assumptions, decisions)
        } else if let Some((PlayData { k: kp, .. }, _)) =
            pl.iter().find(|(PlayData { pos: cp, .. }, _)| cp == &play_data.pos)
        {
            let p = match self.counter_le_p(kp, &play_data.k, &Player::Eve) {
                true => Player::Eve,
                // It is guaranteed that either kp < k for Eve or kp < k for Adam
                false => Player::Adam,
            };
            assumptions.get_mut_p(&p).insert(
                PlayData { pos: play_data.pos.clone(), k: kp.clone() },
                Instant::now(),
            );
            self.backtrack(p, play_data.pos, pl, assumptions, decisions)
        } else {
            let kp = Rc::new(Self::counter_next(
                &play_data.k,
                Position::priority(&play_data.pos),
            ));

            let mut pi = match play_data.pos.clone() {
                Position::Adam(pos) => (Self::universal_move(pos), kp.clone()),
                Position::Eve(b_i) => (
                    Self::exists_move(
                        self.get_formula(&b_i).clone(),
                        self.fix_system.len(),
                    ),
                    kp.clone(),
                ),
            };
            let pp =
                PlayData { pos: pi.0.next().unwrap().clone(), k: kp.clone() };
            pl.push((play_data, pi));

            self.explore(pp, pl, assumptions, decisions)
        }
    }

    fn backtrack(
        &self,
        p: Player,
        c: Position,
        mut pl: Playlist,
        mut assumptions: PositionCounterSet<Instant>,
        mut decisions: PositionCounterSet<(Justification, Instant)>,
    ) -> Player {
        if let Some((play_data, mut pi)) = pl.pop() {
            let cp = &play_data.pos;
            let kp = &play_data.k;

            if let (Some(pos), true) =
                (pi.0.next(), Position::get_controller(&cp) != p)
            {
                let pp = PlayData { pos, k: pi.1.clone() };
                pl.push((play_data, pi));
                self.explore(pp, pl, assumptions, decisions)
            } else {
                if Position::get_controller(&cp) == p {
                    decisions.get_mut_p(&p).insert(
                        play_data.clone(),
                        (
                            Justification::SetOfMoves(HashSet::from([
                                c.clone()
                            ])),
                            Instant::now(),
                        ),
                    );
                } else {
                    decisions.get_mut_p(&p).insert(
                        PlayData { pos: cp.clone(), k: kp.clone() },
                        (
                            match &cp {
                                Position::Eve(_) => {
                                    // let _ = self.get_formula(b_i);
                                    Justification::SetOfMoves(
                                        HashSet::new(), // self.exists_move(formula)
                                                        //     .unwrap_or_default()
                                                        //     .into_iter()
                                                        //     .map(|x| Position::Adam(x))
                                                        //     .collect(),
                                    )
                                }
                                Position::Adam(_) => Justification::SetOfMoves(
                                    HashSet::new(), // self.universal_move(x)
                                                    //     .into_iter()
                                                    //     .map(|b_i| Position::Eve(b_i))
                                                    //     .collect(),
                                ),
                            },
                            Instant::now(),
                        ),
                    );
                }
                assumptions.get_mut_p(&p).remove(&play_data);
                let opponent = Player::get_opponent(&p);
                if let Some(_) = assumptions.get_p(&opponent).get(&play_data) {
                    Self::forget(
                        &opponent,
                        &play_data,
                        &mut assumptions,
                        &mut decisions,
                    );
                    assumptions.get_mut_p(&opponent).remove(&play_data);
                };

                self.backtrack(p, play_data.pos, pl, assumptions, decisions)
            }
        } else {
            p.clone()
        }
    }

    #[inline]
    fn forget(
        p: &Player,
        c: &PlayData,
        assumptions: &PositionCounterSet<Instant>,
        decisions: &mut PositionCounterSet<(Justification, Instant)>,
    ) {
        let after_not_valid =
            &mut assumptions.get_p(p).get(c).cloned().unwrap();
        decisions.get_mut_p(p).retain(|_, (_, inst)| inst < after_not_valid);
    }

    #[inline]
    pub fn get_formula(&self, c: &EvePos) -> &LogicFormula {
        let EvePos { b, i } = c;
        if let Some(f) = self.symbolic_moves.iter().find(
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

    fn exists_move(
        f: LogicFormula,
        m: usize,
    ) -> Box<dyn Iterator<Item = Position>> {
        match f {
            LogicFormula::False => Box::new(std::iter::empty()),
            LogicFormula::True => Box::new(std::iter::once(Position::Adam(AdamPos { x: vec![] }))),
            LogicFormula::BasisElem(b, i) => {
                let mut x =
                    vec![BTreeSet::<String>::new(); m];
                x[i - 1].insert(b.clone());
                Box::new(std::iter::once(Position::Adam(AdamPos { x })))
            }
            LogicFormula::Conj(fs) =>
                Box::new(fs.into_iter()
                    .map(|phi_k| {
                        Self::exists_move(phi_k, m)
                            .collect::<Vec<_>>()
                    })
                    .multi_cartesian_product()
                    .map(move |y| {
                        Position::Adam(AdamPos {
                        x: y.into_iter().fold(vec![BTreeSet::new(); m], |acc, pos| {
                            match pos {
                                Position::Adam(AdamPos { x }) =>
                                    acc.into_iter()
                                        .zip(x)
                                        .map(|(e, ep): (BTreeSet<_>, BTreeSet<_>)| {
                                            e.union(&ep)
                                                .cloned()
                                                .collect::<BTreeSet<String>>()
                                        })
                                        .collect::<Vec<_>>(),
                                Position::Eve(_) => panic!("Position not expected here.")
                            }})
                        })
                    })),

            LogicFormula::Disj(fs) =>
                Box::new(fs.into_iter()
                // Symbolic moves are simplified, thus this is ok
                    .flat_map(move |phi_k| {
                        Self::exists_move(phi_k, m)
                    }),
            ),
        }
    }

    #[inline]
    fn universal_move(
        AdamPos { x }: AdamPos,
    ) -> Box<dyn Iterator<Item = Position>> {
        Box::new(x.into_iter().enumerate().flat_map(|(i, x_i)| {
            x_i.into_iter()
                .map(move |b| Position::Eve(EvePos { b: b.clone(), i: i + 1 }))
        }))
    }

    #[inline]
    pub fn contains(
        &self,
        decisions: &PositionCounterSet<(Justification, Instant)>,
        PlayData { pos: c, k }: &PlayData,
    ) -> Option<Player> {
        if decisions.get_p(&Player::Eve).iter().any(
            |(PlayData { pos: cp, k: kp }, _)| {
                self.counter_leq_p(kp, k, &Player::Eve) && cp == c
            },
        ) {
            Some(Player::Eve)
        } else if decisions.get_p(&Player::Adam).iter().any(
            |(PlayData { pos: cp, k: kp }, _)| {
                self.counter_leq_p(kp, k, &Player::Adam) && cp == c
            },
        ) {
            Some(Player::Adam)
        } else {
            None
        }
    }

    #[inline]
    fn counter_le_eve(&self, k: &Counter, kp: &Counter) -> bool {
        let n = k.iter().zip(kp).enumerate().rev().find(|(_, (n, np))| n != np);
        if let Some((i, _)) = n {
            match self.fix_system[i].fix_ty {
                FixType::Max => k[i] < kp[i],
                FixType::Min => k[i] > kp[i],
            }
        } else {
            false
        }
    }

    #[inline]
    fn counter_le_p(&self, k: &Counter, kp: &Counter, p: &Player) -> bool {
        match p {
            Player::Eve => self.counter_le_eve(k, kp),
            Player::Adam => self.counter_le_eve(kp, k),
        }
    }

    /// Implements a total order for the counter:
    ///
    ///  - it is the case that `k < k'` for the existential player, whenever
    ///    the largest `i` such that `k_i != k'_i` is the
    ///    index of a greatest fixpoint, that is to say whenever
    ///    `self.fix_types[i]` has value `Max`, or if the value is
    ///    `Min` and `k_i > k'_i`,
    ///  - we say `k < k'` for the universal player whenever it is not true that
    ///    $k' < k$ for the existential player,
    ///  - `k <= k'` for a player whenever `k < k'` or $k = k'$, for a player.
    #[inline]
    pub fn counter_leq_p(&self, k: &Counter, kp: &Counter, p: &Player) -> bool {
        k == kp || self.counter_le_p(k, kp, p)
    }

    /// Updates the counter, such that each non-zero priority (argument `i`) is associated
    /// with the number of times the priority has been encountered in the play
    /// since a higher priority was last faced. Note that:
    ///
    ///  - `next(k, 0) = k`
    ///  - `next(k, i) = k'`.
    ///
    #[inline]
    fn counter_next(k: &Counter, i: usize) -> Counter {
        if i == 0 {
            k.clone()
        } else {
            let i = i - 1;
            let mut kp = vec![0; k.len()];
            kp[i] = k[i] + 1;
            for j in i + 1..kp.len() {
                kp[j] = k[j]
            }
            kp
        }
    }
}