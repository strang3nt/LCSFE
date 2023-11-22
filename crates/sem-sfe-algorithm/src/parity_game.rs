mod play_data;
pub mod player;
pub mod position;
mod position_counter_set;

use std::collections::BTreeSet;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Instant;

use crate::ast::fixpoint_system::{FixEq, FixType};
use crate::ast::symbolic_moves_dag::{BasisElem, FormulaOperator, Node, SymbolicExistsMoves};
use play_data::PlayData;
use player::Player;
use position::{AdamPos, EvePos, Position};
use position_counter_set::PositionCounterSet;

enum AltMoves {
    Adam(Box<dyn Iterator<Item = Position>>),
    Eve(Rc<Node<FormulaOperator>>),
}

type Playlist = Vec<(PlayData, (AltMoves, Rc<Counter>))>;
type Counter = Vec<u32>;

pub struct LocalAlgorithm<'a> {
    pub fix_system: &'a [FixEq],
    pub symbolic_moves: &'a SymbolicExistsMoves,
}

impl<'a> LocalAlgorithm<'a> {
    pub fn local_check(&self, b: String, i: usize) -> Player {
        let m: usize = self.fix_system.len();
        let c = Position::Eve(EvePos { b: self.symbolic_moves.get_basis_usize(&b), i });
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
        mut decisions: PositionCounterSet<Instant>,
    ) -> Player {
        if self.is_empty(&play_data) {
            let opponent = Player::get_opponent(&Position::get_controller(&play_data.pos));
            decisions.get_mut_p(&opponent).insert(play_data, Instant::now());
            self.backtrack(opponent, pl, assumptions, decisions)
        } else if let Some(p) = self.contains(&decisions, &play_data) {
            self.backtrack(p, pl, assumptions, decisions)
        } else if let Some((PlayData { k: kp, .. }, _)) =
            pl.iter().find(|(PlayData { pos: cp, .. }, _)| cp == &play_data.pos)
        {
            let p = match self.counter_le_p(kp, &play_data.k, &Player::Eve) {
                true => Player::Eve,
                // It is guaranteed that either kp < k for Eve or kp < k for Adam
                false => Player::Adam,
            };
            assumptions
                .get_mut_p(&p)
                .insert(PlayData { pos: play_data.pos, k: kp.clone() }, Instant::now());
            self.backtrack(p, pl, assumptions, decisions)
        } else {
            let kp = Rc::new(Self::counter_next(&play_data.k, Position::priority(&play_data.pos)));

            match &play_data.pos {
                Position::Adam(x) => {
                    let mut moves = Self::universal_move(x.clone());
                    let pp = PlayData { pos: moves.next().unwrap(), k: kp.clone() };
                    pl.push((play_data, (AltMoves::Adam(moves), kp)));
                    self.explore(pp, pl, assumptions, decisions)
                }
                Position::Eve(x @ EvePos { b, i }) => {
                    let f = self.symbolic_moves.get_formula(*b, *i);
                    let (new_formula, new_assumpt) =
                        self.reduce(&f, &kp, &decisions, (x, &play_data.k), &pl);
                    assumptions.union(new_assumpt);
                    if let Some(new_pos) = self.next_move(&new_formula) {
                        pl.push((play_data, (AltMoves::Eve(new_formula), kp.clone())));
                        let next_move = PlayData { pos: Position::Adam(new_pos), k: kp };
                        self.explore(next_move, pl, assumptions, decisions)
                    } else {
                        decisions.get_mut_p(&Player::Adam).insert(play_data, Instant::now());
                        self.backtrack(Player::Adam, pl, assumptions, decisions)
                    }
                }
            }
        }
    }

    #[inline(always)]
    fn is_empty(&self, PlayData { pos: c, .. }: &PlayData) -> bool {
        match c {
            Position::Eve(EvePos { b, i }) => {
                SymbolicExistsMoves::is_formula_false(&self.symbolic_moves.get_formula(*b, *i))
            }
            Position::Adam(AdamPos { x }) => x.iter().all(BTreeSet::is_empty),
        }
    }

    fn backtrack(
        &self,
        p: Player,
        mut pl: Playlist,
        mut assumptions: PositionCounterSet<Instant>,
        mut decisions: PositionCounterSet<Instant>,
    ) -> Player {
        if let Some((play_data, pi)) = pl.pop() {
            if let (Some(pos), Some(pip)) = match (&play_data.pos, pi) {
                (_, (AltMoves::Adam(mut it), kp))
                    if Position::get_controller(&play_data.pos) != p =>
                {
                    if let Some(pos) = it.next() {
                        (Some(pos), Some((AltMoves::Adam(Box::new(it)), kp.clone())))
                    } else {
                        (None, None)
                    }
                }
                (Position::Eve(x), (AltMoves::Eve(f), kp))
                    if Position::get_controller(&play_data.pos) != p =>
                {
                    let (fp, new_assumpt) =
                        self.reduce(&f, &kp, &decisions, (&x, &play_data.k), &pl);
                    assumptions.union(new_assumpt);

                    if let Some(new_pos) = self.next_move(&fp).map(Position::Adam) {
                        (Some(new_pos), Some((AltMoves::Eve(fp), kp.clone())))
                    } else {
                        (None, None)
                    }
                }
                _ => (None, None),
            } {
                let k = pip.1.clone();
                pl.push((play_data, pip));
                self.explore(PlayData { pos, k }, pl, assumptions, decisions)
            } else {
                let decision_time = Instant::now();
                let opponent = Player::get_opponent(&p);
                if let Some(after_not_valid) = assumptions.get_mut_p(&opponent).get_mut(&play_data)
                {
                    Self::forget(&opponent, after_not_valid, &mut decisions);
                    assumptions.get_mut_p(&opponent).remove(&play_data);
                };
                assumptions.get_mut_p(&p).remove(&play_data);
                decisions.get_mut_p(&p).insert(play_data, decision_time);
                self.backtrack(p, pl, assumptions, decisions)
            }
        } else {
            println!(
                "{} assumption: {:?}\n{} assumption: {:?}",
                &p,
                assumptions.get_p(&p),
                &Player::get_opponent(&p),
                assumptions.get_p(&Player::get_opponent(&p))
            );
            p
        }
    }

    #[inline(always)]
    fn forget(
        p: &Player,
        after_not_valid: &mut Instant,
        decisions: &mut PositionCounterSet<Instant>,
    ) {
        decisions.get_mut_p(p).retain(|_, inst| inst <= after_not_valid);
    }

    #[inline(always)]
    fn universal_move(AdamPos { x }: AdamPos) -> Box<dyn Iterator<Item = Position>> {
        Box::new(
            x.into_iter()
                .enumerate()
                .flat_map(|(i, x_i)| x_i.into_iter().map(move |b| Position::Eve(EvePos { b, i }))),
        )
    }

    #[inline(always)]
    pub fn contains(
        &self,
        decisions: &PositionCounterSet<Instant>,
        PlayData { pos: c, k }: &PlayData,
    ) -> Option<Player> {
        if decisions.get_p(&Player::Adam).iter().any(|(PlayData { pos: cp, k: kp }, _)| {
            cp == c && self.counter_leq_p(kp, k, &Player::Adam)
        }) {
            Some(Player::Adam)
        } else if decisions.get_p(&Player::Eve).iter().any(|(PlayData { pos: cp, k: kp }, _)| {
            cp == c && self.counter_leq_p(kp, k, &Player::Eve)
        }) {
            Some(Player::Eve)
        } else {
            None
        }
    }

    #[inline(always)]
    fn counter_le_eve(&self, k: &[u32], kp: &[u32]) -> bool {
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

    #[inline(always)]
    fn counter_le_p(&self, k: &[u32], kp: &[u32], p: &Player) -> bool {
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
    ///    `self.fix_types[i]` has value `Max` and `k_i < k'_i`, or if the value
    ///    is `Min` and `k_i > k'_i`,
    ///  - we say `k < k'` for the universal player whenever we have
    ///    $k' < k$ for the existential player,
    ///  - `k <= k'` for a player whenever `k < k'` or $k = k'$, for a player.
    #[inline(always)]
    fn counter_leq_p(&self, k: &[u32], kp: &[u32], p: &Player) -> bool {
        k == kp || self.counter_le_p(k, kp, p)
    }

    /// Updates the counter, such that each non-zero priority (argument `i`) is associated
    /// with the number of times the priority has been encountered in the play
    /// since a higher priority was last faced. Note that:
    ///
    ///  - `next(k, 0) = k`
    ///  - `next(k, i) = [0_0_{i-1}, k_i + 1, k_{i+1}_m]`.
    ///
    #[inline(always)]
    fn counter_next(k: &[u32], i: usize) -> Counter {
        if i == 0 {
            k.to_vec()
        } else {
            let i = i - 1;
            let mut kp = vec![0; k.len()];
            kp[i] = k[i] + 1;
            kp[(i + 1)..].copy_from_slice(&k[(i + 1)..]);
            kp
        }
    }

    #[inline(always)]
    fn reduce(
        &self,
        f: &Rc<Node<FormulaOperator>>,
        k: &Rc<Vec<u32>>,
        decisions: &PositionCounterSet<Instant>,
        last_move: (&EvePos, &Rc<Vec<u32>>),
        pl: &Playlist,
    ) -> (Rc<Node<FormulaOperator>>, PositionCounterSet<Instant>) {
        let (fp, new_assumpt) = self.apply_decisions(f, k, decisions, last_move, pl);
        (self.symbolic_moves.simplify(fp), new_assumpt)
    }

    fn apply_decisions(
        &self,
        f: &Rc<Node<FormulaOperator>>,
        k: &Rc<Vec<u32>>,
        decisions: &PositionCounterSet<Instant>,
        (eve_pos, kp): (&EvePos, &Rc<Vec<u32>>),
        pl: &Playlist,
    ) -> (Rc<Node<FormulaOperator>>, PositionCounterSet<Instant>) {
        match f.deref() {
            Node { val: FormulaOperator::Atom(BasisElem { b, i }), .. } => {
                if let Some(player) = self.contains(
                    decisions,
                    &PlayData { pos: Position::Eve(EvePos { b: *b, i: *i }), k: k.clone() },
                ) {
                    (
                        if Player::Eve == player {
                            self.symbolic_moves.get_true_atom()
                        } else {
                            self.symbolic_moves.get_false_atom()
                        },
                        PositionCounterSet::new(),
                    )
                } else if let Some((play_data, _)) =
                    pl.iter().find(|(PlayData { pos, k: kp }, _)| {
                        matches!(pos, Position::Eve(EvePos { b: bp, i: ip }) if bp == b
                        && i == ip
                        && self.counter_le_eve(kp, &k))
                    })
                {
                    let mut new_assumpt = PositionCounterSet::new();
                    new_assumpt.get_mut_p(&Player::Eve).insert(play_data.clone(), Instant::now());
                    (self.symbolic_moves.get_true_atom(), new_assumpt)
                } else if let Some((play_data, _)) =
                    pl.iter().find(|(PlayData { pos, k: kp }, _)| {
                        matches!(pos, Position::Eve(EvePos { b: bp, i: ip }) if bp == b
                        && i == ip
                        && self.counter_le_eve(&k, kp))
                    })
                {
                    let mut new_assumpt = PositionCounterSet::new();
                    new_assumpt.get_mut_p(&Player::Adam).insert(play_data.clone(), Instant::now());
                    (self.symbolic_moves.get_false_atom(), new_assumpt)
                } else if &eve_pos.b == b && i == &eve_pos.i && self.counter_le_eve(&k, &kp) {
                    let mut new_assumpt = PositionCounterSet::new();
                    new_assumpt.get_mut_p(&Player::Adam).insert(
                        PlayData {
                            pos: Position::Eve(EvePos { b: eve_pos.b, i: eve_pos.i }),
                            k: kp.clone(),
                        },
                        Instant::now(),
                    );
                    (self.symbolic_moves.get_false_atom(), new_assumpt)
                } else if &eve_pos.b == b && i == &eve_pos.i && self.counter_le_eve(&kp, &k) {
                    let mut new_assumpt = PositionCounterSet::new();
                    new_assumpt.get_mut_p(&Player::Eve).insert(
                        PlayData {
                            pos: Position::Eve(EvePos { b: eve_pos.b, i: eve_pos.i }),
                            k: kp.clone(),
                        },
                        Instant::now(),
                    );
                    (self.symbolic_moves.get_true_atom(), new_assumpt)
                } else {
                    (f.clone(), PositionCounterSet::new())
                }
            }
            Node { children, .. } if children.is_empty() => (f.clone(), PositionCounterSet::new()),
            Node { val, children } => {
                let mut new_assumpts = PositionCounterSet::new();
                let mut new_formula_args = Vec::with_capacity(children.len());
                for x_j in children {
                    let (new_formula_j, new_assumpts_j) =
                        self.apply_decisions(x_j, k, decisions, (eve_pos, kp), pl);
                    new_assumpts.union(new_assumpts_j);
                    new_formula_args.push(new_formula_j);
                }
                (Rc::new(Node { val: val.clone(), children: new_formula_args }), new_assumpts)
            }
        }
    }

    #[inline(always)]
    fn next_move(&self, f: &Rc<Node<FormulaOperator>>) -> Option<AdamPos> {
        match f.deref() {
            Node { val: FormulaOperator::Or, children } if children.is_empty() => None,
            Node { val: FormulaOperator::And, children } if children.is_empty() => {
                Some(AdamPos { x: vec![BTreeSet::default(); self.fix_system.len()] })
            }
            _ => Some(self.build_next_move(f)),
        }
    }

    fn build_next_move(&self, f: &Rc<Node<FormulaOperator>>) -> AdamPos {
        let mut c: Vec<BTreeSet<usize>> = vec![BTreeSet::default(); self.fix_system.len()];
        match f.deref() {
            Node { val: FormulaOperator::Atom(BasisElem { b, i }), .. } => {
                c[*i].insert(*b);
            }
            Node { val: FormulaOperator::And, children } if !children.is_empty() => {
                children.iter().for_each(|j| {
                    let AdamPos { x } = self.build_next_move(j);
                    for i in 0..c.len() {
                        c[i].extend(x[i].clone());
                    }
                })
            }
            Node { val: FormulaOperator::Or, children } if !children.is_empty() => {
                let AdamPos { x } = self.build_next_move(&children[0]);
                c = x;
            }
            _ => panic!("Atom of type True, or False not expected here"),
        };
        AdamPos { x: c }
    }
}
