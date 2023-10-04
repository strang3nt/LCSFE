use std::collections::BTreeSet;
use std::time::Instant;
use std::{collections::HashMap, collections::HashSet};

use super::{player::Player, position::Position};
use crate::parser::symbolic_exists_moves::{
    LogicFormula, SymbolicExistsMove, SymbolicSystem,
};

use itertools::Itertools;

#[derive(Debug)]
pub enum FixpointType {
    Max,
    Min,
}

type Playlist = Vec<(Position, Counter, HashSet<UnexploredMove>)>;
type Counter = Vec<u32>;

impl SymbolicSystem {
    pub fn get_formula(&self, c: (&String, &usize)) -> &LogicFormula {
        let formulas = &self.0;

        if let Some(f) = formulas.iter().find(
            |SymbolicExistsMove { formula: _, base_elem, func_name }| {
                base_elem == c.0 && func_name == c.1
            },
        ) {
            &f.formula
        } else {
            panic!("Define a symbolic exists move for function {}, w.r.t. base element {}", c.1, c.0)
        }
    }
}

pub struct ParityGame {
    pub fix_types: Vec<FixpointType>,
    pub symbolic_moves: SymbolicSystem,
    pub base: Vec<String>,
}
impl ParityGame {
    pub fn local_check(&self, c: Position) -> Player {
        println!(
            "The parameters are: \n {:?} \n{:?}\n {:?}",
            self.fix_types, self.symbolic_moves, self.base
        );

        let m: usize = self.fix_types.len();

        self.explore(
            &c,
            &vec![0; m + 1],
            &mut vec![],
            &mut Assumptions::new(),
            &mut Decisions::new(),
        )
    }

    fn explore(
        &self,
        c: &Position,
        k: &Counter,
        pl: &mut Playlist,
        asmt: &mut Assumptions,
        dec: &mut Decisions,
    ) -> Player {
        if self.is_empty(&c) {
            let opponent = Player::get_opponent(&Position::get_controller(&c));
            dec.push(&opponent, c.clone(), k.clone(), Justification::Truth);
            self.backtrack(&opponent, c, pl, asmt, dec)
        } else if let Some(p) = dec.contains(self, c, &k) {
            self.backtrack(&p, c, pl, asmt, dec)
        } else if let Some((_, kp, _)) = pl.iter().find(|(cp, _, _)| cp == c) {
            let p = match self.le(&kp, &k, &Player::Eve) {
                true => Player::Eve,
                false => Player::Adam,
            };
            asmt.update(p.clone(), c.clone(), kp.clone());
            self.backtrack(&p, &c, pl, asmt, dec)
        } else {
            let kp = Self::next(k, Self::priority(c));
            let mut pi = match c {
                Position::Adam(pos) => self
                    .universal_move(&pos)
                    .iter()
                    .map(|pos| (UnexploredMove(pos.clone(), kp.clone())))
                    .collect::<HashSet<_>>(),
                Position::Eve(b, i) => {
                    let formula = self.symbolic_moves.get_formula((&b, &i));
                    self.existential_move(formula)
                        .unwrap()
                        .into_iter()
                        .map(|x| {
                            UnexploredMove(Position::Adam(x), kp.clone())
                        })
                        .collect::<HashSet<_>>()
                }
            };

            let cp = pi.iter().next().unwrap().clone();
            println!("{:?}", pi);
            pi.remove(&cp);
            println!("{:?}", &cp);
            pl.push((c.clone(), k.clone(), pi));
            self.explore(&cp.0, &kp, pl, asmt, dec)
        }
    }

    fn backtrack(
        &self,
        p: &Player,
        c: &Position,
        pl: &mut Playlist,
        asmt: &mut Assumptions,
        dec: &mut Decisions,
    ) -> Player {
        if let Some((cp, kp, mut pi)) = pl.pop() {

            if &Position::get_controller(&cp) != p && !pi.is_empty() {
                match &pi.iter().next().unwrap().clone() {
                    i @ UnexploredMove(cs, ks) => {
                        pi.remove(i);
                        pl.push((cp, kp, pi));
                        self.explore(cs, ks, pl, asmt, dec)
                    }
                }
            } else {
                if Position::get_controller(&cp) == *p {
                    dec.push(
                        p,
                        cp.clone(),
                        kp.clone(),
                        Justification::SetOfMoves(vec![c.clone()]),
                    );
                } else {
                    match &cp {
                        Position::Eve(str, i) => {
                            let formula =
                                self.symbolic_moves.get_formula((&str, &i));
                            dec.push(
                                &Player::Eve,
                                cp.clone(),
                                kp.clone(),
                                Justification::Formula(formula.clone()),
                            );
                        }
                        Position::Adam(btree) => {
                            dec.push(
                                &Player::Adam,
                                cp.clone(),
                                kp.clone(),
                                Justification::SetOfMoves(
                                    self.universal_move(&btree),
                                ),
                            );
                        }
                    }
                }

                asmt.remove(p, &cp, &kp);
                let opponent = Player::get_opponent(p);
                if let Some(_) = asmt.find(&opponent, &cp, &kp) {
                    self.forget(&opponent, dec, asmt, &cp, &kp);
                    asmt.remove(&opponent, &cp, &kp);
                };

                self.backtrack(&p, &cp, pl, asmt, dec)
            }
        } else {
            p.clone()
        }
    }

    fn forget(
        &self,
        p: &Player,
        d: &mut Decisions,
        g: &Assumptions,
        c: &Position,
        k: &Counter,
    ) {
        if let Some(time) = g.find(p, c, k) {
            d.forget(p, c, k, &mut time.clone());
        }
    }

    fn is_empty(&self, p: &Position) -> bool {
        match p {
            Position::Eve(b, i) => {
                self.symbolic_moves.get_formula((b, i)) == &LogicFormula::False
            }
            Position::Adam(v) => Vec::is_empty(&self.universal_move(v)),
        }
    }

    /// Moves are supposed to be normalized
    fn existential_move(
        &self,
        f: &LogicFormula,
    ) -> Option<HashSet<Vec<BTreeSet<String>>>> {
        let mut c: HashSet<Vec<BTreeSet<String>>> = HashSet::new();

        match f {
            LogicFormula::False => None,
            LogicFormula::True => Some(c),
            LogicFormula::BaseElem(b, i) => {
                let mut x =
                    vec![BTreeSet::<String>::new(); self.fix_types.len()];
                x[*i - 1].insert(b.clone());
                c.insert(x);
                Some(c)
            }
            LogicFormula::Conj(fs) => {
                Some(
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
                                vec![BTreeSet::new(); self.fix_types.len()],
                                |acc, elem| {
                                    acc.into_iter()
                                    .enumerate()
                                    .map(|(i, e): (usize, BTreeSet<String>)| {
                                        e.union(&elem[i])
                                            .map(|x| x.clone())
                                            .collect::<BTreeSet<String>>()
                                    }).collect::<Vec<_>>()
                                },
                            )
                        })
                        .collect::<HashSet<Vec<BTreeSet<_>>>>(),
                )
            } // unwrap or else none,
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

    fn universal_move(
        &self,
        univ_position: &Vec<BTreeSet<String>>,
    ) -> Vec<Position> {
        univ_position.iter().enumerate().fold(vec![], |mut acc, (i, x_i)| {
            for b in x_i {
                acc.push(Position::Eve(b.clone(), i + 1));
            };
            acc
        })
    }

    fn priority(c: &Position) -> usize {
        match c {
            Position::Eve(_, i) => *i,
            Position::Adam(_) => 0,
        }
    }

    /// TODO eliminate recursion
    /// A pre-condition of this function is that the argument `f: &LogicFormula` does
    /// not have `LogicFormula::True` or `Logic::Formula::False` leaves.
    fn build_next_move(&self, f: &LogicFormula) -> Vec<BTreeSet<String>> {
        let mut c: Vec<BTreeSet<String>> =
            vec![BTreeSet::new(); self.fix_types.len()];

        match f {
            LogicFormula::BaseElem(b, i) => {
                c[i.clone()].insert(b.clone());
            }
            LogicFormula::Conj(fs) => {
                for f in fs {
                    let mut j = 0;
                    for fj in self.build_next_move(f) {
                        c[j].extend(fj);
                        j = j + 1;
                    }
                }
            }
            LogicFormula::Disj(fs) => c = self.build_next_move(&fs[0]),
            _ => panic!("Formula {:?} has true or false leaves", f),
        };
        c
    }

    /// Precondition: the formula `f` is simplified, using the function `reduce(f)`,
    /// and `nextMove(f)` has never been called before.
    /// TODO: values of logic formula true or false should return different values.
    fn next_move(&self, f: &LogicFormula) -> Option<Vec<BTreeSet<String>>> {
        match f {
            LogicFormula::False => None,
            LogicFormula::True => Some(vec![]),
            _ => Some(self.build_next_move(f)),
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
            let result_for_eve = match self.fix_types[n - 1] {
                FixpointType::Max => k[n] < kp[n],
                FixpointType::Min => k[n] > kp[n],
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

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct UnexploredMove(Position, Counter);

struct Assumptions {
    eve: HashMap<(Position, Counter), Instant>,
    adam: HashMap<(Position, Counter), Instant>,
}

impl Assumptions {
    pub fn new() -> Assumptions {
        Assumptions { eve: HashMap::new(), adam: HashMap::new() }
    }

    pub fn update(&mut self, p: Player, c: Position, k: Counter) {
        match p {
            Player::Eve => self.eve.insert((c, k), Instant::now()),
            Player::Adam => self.adam.insert((c, k), Instant::now()),
        };
    }

    pub fn remove(&mut self, p: &Player, c: &Position, k: &Counter) {
        match p {
            Player::Eve => self.eve.remove(&(c.clone(), k.clone())),
            Player::Adam => self.adam.remove(&(c.clone(), k.clone())),
        };
    }

    pub fn find(
        &self,
        p: &Player,
        c: &Position,
        k: &Counter,
    ) -> Option<Instant> {
        match p {
            Player::Eve => {
                self.eve.get(&(c.clone(), k.clone())).map(|v| v.clone())
            }
            Player::Adam => {
                self.adam.get(&(c.clone(), k.clone())).map(|v| v.clone())
            }
        }
    }
}

enum Justification {
    Truth,
    SetOfMoves(Vec<Position>),
    Formula(LogicFormula),
}

struct Decisions {
    eve: HashMap<(Position, Counter), (Justification, Instant)>,
    adam: HashMap<(Position, Counter), (Justification, Instant)>,
}

impl Decisions {
    pub fn new() -> Decisions {
        Decisions { eve: HashMap::new(), adam: HashMap::new() }
    }

    pub fn push(
        &mut self,
        p: &Player,
        pos: Position,
        k: Counter,
        j: Justification,
    ) {
        match p {
            &Player::Eve => self.eve.insert((pos, k), (j, Instant::now())),
            &Player::Adam => self.adam.insert((pos, k), (j, Instant::now())),
        };
    }

    pub fn contains(
        &self,
        game: &ParityGame,
        c: &Position,
        k: &Counter,
    ) -> Option<Player> {
        if self.eve.iter().any(|((pos, kp), _)| game.leq(kp, k, &Player::Eve) && pos == c)
        {
            Some(Player::Eve)
        } else if self
            .adam
            .iter()
            .any(|((pos, kp), _)| game.leq(kp, k, &Player::Adam) && pos == c)
        {
            Some(Player::Adam)
        } else {
            None
        }
    }

    pub fn forget(
        &mut self,
        p: &Player,
        c: &Position,
        k: &Counter,
        after: &mut Instant,
    ) {
        match p {
            Player::Eve => {
                self.eve.retain(|_, (_, inst)| inst < after);
            }
            Player::Adam => {
                self.adam.retain(|_, (_, inst)| inst < after);
            }
        }
    }
}
