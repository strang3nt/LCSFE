use std::collections::BTreeSet;
use std::{collections::HashMap, collections::HashSet};
use std::time::Instant;

use super::{
    position::Position, 
    player::Player, 
    counter::Counter};
use crate::parser::symbolic_exists_moves::{SymbolicExistsMove, SymbolicSystem, LogicFormula};

pub enum FixpointType {
    Max,
    Min,
}

impl SymbolicSystem {
    pub fn get_formula(&self, c :  (&String, &usize)) -> &LogicFormula {
        let formulas = &self.0;

        if let Some(f) = formulas
            .iter()
            .find(|SymbolicExistsMove { 
                formula, 
                base_elem, 
                func_name }| {
            base_elem == c.0 && func_name == c.1
        }) { &f.formula } else { panic!("Define a symbolic exists move for function {}, w.r.t. base element {}", c.1, c.0) }
    }
}

pub struct ParityGame
{
    fix_types: Vec<FixpointType>,
    symbolic_moves: SymbolicSystem,
    base: Vec<String>,
}
impl ParityGame {

    pub fn local_check() -> Player {
        unimplemented!()
    }

    fn explore(
        &self,
        c: &Position,
        k: &Counter, 
        r: &mut Playlist, 
        g: &mut Assumptions,
        d: &mut Decisions,
    ) -> Player {

        if self.is_empty(&c) {
            let opponent = Player::get_opponent(&Position::get_controller(&c));
            g.update(&opponent, &c, &k);
            self.backtrack(&opponent, c, r, g, d)

        } else if let Some(p) = d.contains(c, &k, &self.fix_types) {
            self.backtrack(&p, c, r, g, d)
        } else if let Some(kp) = r.contains_position(&c, &k) {
            let may_existential_win = Counter::leq_p(&kp, &k, &Player::Existential, &self.fix_types);
            let p = if may_existential_win { Player::Existential } else { Player::Universal };
            g.update(&p, &c, &kp);
            self.backtrack(&p, &c, r, g, d)
        } else {
            match c {
                Position::Universal(univ_position) => {
                    let mut positions: Vec<Position> = self.universal_move(&univ_position);
                    let kp = k.next(Self::priority(c));
                    let curr_move = positions.pop();
                    let pi: HashSet<_> = positions.iter().map(|pos| UnexploredMoves::Universal(pos.clone(), kp.clone())).collect();
                    r.push(c.clone(), k.clone(), pi);
                    self.explore(&curr_move.unwrap(), &kp, r, g, d)
                }
                Position::Existential(b, i) => {

                    let formula = self.symbolic_moves.get_formula((&b, &i));
                    let cp = self.next_move(formula);
                    let kp = k.next(Self::priority(c));
                    let mut pi = HashSet::new();
                    pi.insert(UnexploredMoves::Existential(formula.clone(), kp.clone()));
                    r.push(c.clone(), k.clone(), pi);
                    self.explore(&Position::Universal(cp.unwrap()), &kp, r, g, d)
                }
            }
        }
    }

    fn backtrack(&self, 
        p: &Player, 
        c: &Position, 
        r: &mut Playlist, 
        g: &mut Assumptions, 
        d: &mut Decisions
    ) -> Player {
        if let Some((cp, kp, mut pi)) = r.0.pop() {

            let unexpl_move = pi.iter().next().unwrap().clone();

            let formula_is_false = match &unexpl_move {
                UnexploredMoves::Existential(LogicFormula::False, _) => false,
                _ => true,
            };

            if Position::get_controller(&cp) != *p && !pi.is_empty() && formula_is_false {                    
                match &unexpl_move {
                    UnexploredMoves::Existential(formula, ks) => {
                        let cs = self.next_move(&formula);
                        r.push(cp.clone(), kp.clone(), pi.clone());
                        self.explore(&Position::Universal(cs.unwrap().clone()), &ks, r, g, d)
                    }

                    i @ UnexploredMoves::Universal(cs, ks) => {
                        pi.remove(&i);
                        r.push(cp.clone(), kp.clone(), pi);
                        self.explore(&cs, ks, r, g, d)
                    }
                }

            } else {
                if Position::get_controller(&cp) == *p {
                    d.push(p, cp.clone(), kp.clone(), Justification::SetOfMoves(vec![c.clone()]));
                } else {
                    match &cp {
                        Position::Existential(str, i) => {
                            let formula = self.symbolic_moves.get_formula( (&str, &i) );
                            d.push(&Player::Existential, cp.clone(), kp.clone(), Justification::Formula(formula.clone()));

                        }
                        Position::Universal(btree) => {
                            
                            d.push(&Player::Universal, cp.clone(), kp.clone(), Justification::SetOfMoves(self.universal_move(&btree)));
                        }
                    }
                }
                
                g.remove(p, &cp, &kp);

                if let Some(_) = g.find(p, &cp, &kp) {
                    self.forget(p, d, g, &cp, &kp);
                    g.remove(&Player::get_opponent(p), &cp, &kp);
                };

                self.backtrack(&p, &cp, r, g, d)
        }

            
        } else {
            p.clone()
        }
    }

    fn forget(&self, p: &Player, d: &mut Decisions, g: &Assumptions, c: &Position, k: &Counter) {
        if let Some(time) = g.find(p, c, k) {
            d.forget(p, c, k, &mut time.clone());
        }

    }

    fn is_empty(&self, p: &Position) -> bool {
        match p {
            Position::Existential(b, i) => 
                *self.symbolic_moves.get_formula((b, i)) == LogicFormula::False,
            Position::Universal(v) => Vec::is_empty(&self.universal_move(v)),
        }
    }

    fn universal_move(&self, univ_position: &Vec<BTreeSet<String>>) -> Vec<Position> {
        let mut p = vec![];
        let m = univ_position.len();
        for i in 1 .. m + 1 {
            for b in &self.base {
                if univ_position[i - 1].contains(b) {
                    p.push(Position::Existential(b.clone(), i))
                }
            }
        }
        p
    }

    fn priority(c: &Position) -> usize {
        match c {
            Position::Existential(_, i) => *i,
            Position::Universal(_) => 0,
        }
    }

    /// TODO eliminate recursion
    /// A pre-condition of this function is that the argument `f: &LogicFormula` does
    /// not have `LogicFormula::True` or `Logic::Formula::False` leaves.
    fn build_next_move(&self, f: &LogicFormula) -> Vec<BTreeSet<String>> {
        let m = self.fix_types.len();
        let mut c: Vec<BTreeSet<String>> = vec![BTreeSet::new(); self.fix_types.len()];

        match f {
            LogicFormula::BaseElem(b, i) => { c[i.clone()].insert(b.clone()); },
            LogicFormula::Conj(fs) => {
                for f in fs {
                    let mut j = 0;
                    for fj in self.build_next_move(f) {
                        c[j].extend(fj);
                        j = j + 1;
                    }
                }},
            LogicFormula::Disj(fs) => { c = self.build_next_move(&fs[0])},
            _ => panic!("Formula {:?} has true or false leaves", f)
        };
        c
    }

    /// Precondition: the formula `f` is simplified, using the function `reduce(f)`,
    /// and `nextMove(f)` has never been called before.
    /// TODO: values of logic formula true or false should return different values.
    fn next_move(&self, f: &LogicFormula) -> Option<Vec<BTreeSet<String>>> {
        match f {
            LogicFormula::False | LogicFormula::True => None,
            _ => Some(self.build_next_move(f)),
        }
    }

    fn reduce(f: &LogicFormula, k: &Counter, dec: Decisions, p: Playlist) -> (LogicFormula, Assumptions, Assumptions){
        unimplemented!()
    }

    fn apply_decisions_and_assumptions(f: &LogicFormula, k: &Counter, dec: Decisions, p: Playlist) -> (LogicFormula, Assumptions, Assumptions) {
        unimplemented!()
    }

    fn unfold(f: &LogicFormula, j: usize, init: bool) -> (LogicFormula, Assumptions, Assumptions) {
        unimplemented!()
    }



}


#[derive(Eq, PartialEq, Hash, Clone)]
enum UnexploredMoves {
    Universal(Position, Counter),
    Existential(LogicFormula, Counter),
}

struct Playlist(Vec<(Position, Counter, HashSet<UnexploredMoves>)>);

impl Playlist {



    pub fn contains_position(&self, c: &Position, k: &Counter) -> Option<Counter> {
        unimplemented!()
    }

    pub fn push(&mut self, p: Position, k: Counter, pi: HashSet<UnexploredMoves>) {
        self.0.push((p, k, pi));
    }

    pub fn isEmpty(&self) -> bool {
        self.0.is_empty()
    }
}

struct Assumptions { 
    
    existential_assumpt: HashMap<(Position, Counter), Instant>,
    universal_assumpt: HashMap<(Position, Counter), Instant>,

}

impl Assumptions {

    pub fn update(&mut self, p: &Player, c: &Position, k: &Counter) {
        match p {
            Player::Existential => self.existential_assumpt.insert((c.clone(), k.clone()), Instant::now()),
            Player::Universal => self.universal_assumpt.insert((c.clone(), k.clone()), Instant::now()),
        };
    }

    pub fn remove(&mut self, p: &Player, c: &Position, k: &Counter) {
        match p {
            Player::Existential => self.existential_assumpt.remove(&(c.clone(), k.clone())),
            Player::Universal => self.universal_assumpt.remove(&(c.clone(), k.clone())),
        };
    }

    pub fn find(&self, p: &Player, c: &Position, k: &Counter) -> Option<Instant> {
        match p {
            Player::Existential => self.existential_assumpt.get(&(c.clone(), k.clone())).map(|v| v.clone()),
            Player::Universal => self.universal_assumpt.get(&(c.clone(), k.clone())).map(|v| v.clone()),
        }
    }
}

enum Justification{
    Truth,
    SetOfMoves(Vec<Position>),
    Formula(LogicFormula),
}

struct Decisions{

    existential_dec: HashMap<(Position, Counter), (Justification, Instant)>,
    universal_dec: HashMap<(Position, Counter), (Justification, Instant)>,

}

impl Decisions {
    
    pub fn push (&mut self, p: &Player, pos: Position, k: Counter, j: Justification) {
        match p {
            &Player::Existential => self.existential_dec.insert((pos, k), (j, Instant::now())),
            &Player::Universal => self.universal_dec.insert((pos, k), (j, Instant::now())),
        };
    }

    pub fn contains(&self, c: &Position, k: &Counter, fix_types: &Vec<FixpointType>) -> Option<Player> {

        if self.existential_dec.iter()
            .any(|((pos, kp), _)| {
                Counter::leq_p(kp, k, &Player::Existential, fix_types)
            }) {
                Some(Player::Existential)
            } else if self.universal_dec.iter()
            .any(|((pos, kp), _)| {
                Counter::leq_p(kp, k, &Player::Universal, fix_types)
            }) { Some(Player::Universal) } else { None }
    }

    pub fn forget(&mut self, p: &Player, c: &Position, k: &Counter, after: &mut Instant) {
        
        match p {
            Player::Existential => { self.existential_dec.retain(|_, (_, inst)| {
                inst < after
            }); },
            Player::Universal => { self.universal_dec.retain(|_, (_, inst)| {
                inst < after
            }); },
        }
        
    }
}
