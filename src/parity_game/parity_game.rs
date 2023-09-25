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
    pub fn get_formula(&self, c : (String, usize)) -> &LogicFormula {
        let formulas = &self.0;

        if let Some(f) = formulas
            .iter()
            .find(|SymbolicExistsMove { 
                formula, 
                base_elem, 
                func_name }| {
            *base_elem == c.0 && *func_name == c.1
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
        c: Position,
        k: Counter, 
        r: &mut Playlist, 
        g: &mut Assumptions,
        d: &mut Decisions,
    ) -> Player {

        // if self.is_empty(&c) {
        //     let opponent = Player::get_opponent(Position::get_controller(&c))
        //     g.update(opponent, c, k);
        //     self.backtrack(opponent, c, r, g, d)

        // } else if d.contains(p, &k, &self.fix_types) {
            
        // }

        unimplemented!()

    }

    fn backtrack(&self, 
        p: Player, 
        c: Position, 
        r: &mut Playlist, 
        g: &mut Assumptions, 
        d: &mut Decisions
    ) -> Player {
        unimplemented!()
    }

    fn is_empty(&self, p: &Position) -> bool {
        match p {
            Position::Existential(b, i) => 
                *self.symbolic_moves.get_formula((b.clone(), *i)) == LogicFormula::False,
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

    fn priority(c: Position) -> usize {
        match c {
            Position::Existential(_, i) => i,
            Position::Universal(_) => 0,
        }
    }

    /// TODO eliminate recursion
    /// A pre-condition of this function is that the argument `f: &LogicFormula` does
    /// not have `LogicFormula::True` or `Logic::Formula::False` leaves.
    fn buildNextMove(f: &LogicFormula, m: usize) -> Vec<HashSet<String>> {
        let mut c: Vec<HashSet<String>> = vec![HashSet::new(); m];

        match f {
            LogicFormula::BaseElem(b, i) => { c[i.clone()].insert(b.clone()); },
            LogicFormula::Conj(fs) => {
                for f in fs {
                    let mut j = 0;
                    for fj in Self::buildNextMove(f, m) {
                        c[j].extend(fj);
                        j = j + 1;
                    }
                }},
            LogicFormula::Disj(fs) => { c = Self::buildNextMove(&fs[0], m)},
            _ => panic!("Formula {:?} has true or false leaves", f)
        };
        c
    }

    /// Precondition: the formula `f` is simplified, using the function `reduce(f)`,
    /// and `nextMove(f)` has never been called before.
    fn nextMove(f: &LogicFormula, m: usize) -> Option<Vec<HashSet<String>>> {
        match f {
            LogicFormula::False | LogicFormula::True => None,
            _ => Some(Self::buildNextMove(f, m)),
        }
    }

    fn reduce(f: &LogicFormula, k: &Counter, dec: Decisions, p: Playlist) -> (LogicFormula, Assumptions, Assumptions){
        unimplemented!()
    }

    fn applyDecisionsAndAssumptions(f: &LogicFormula, k: &Counter, dec: Decisions, p: Playlist) -> (LogicFormula, Assumptions, Assumptions) {
        unimplemented!()
    }

    fn unfold(f: &LogicFormula, j: usize, init: bool) -> (LogicFormula, Assumptions, Assumptions) {
        unimplemented!()
    }



}

struct Playlist(pub Vec<(Position, Counter, HashSet<Position>)>);

impl Playlist {
    pub fn contains(&self, c: Position, k: &Counter, pi: &HashSet<Position>) -> bool {
        unimplemented!()
    }
}

struct Assumptions { 
    
    existential_assumpt: HashMap<(Position, Counter), Instant>,
    universal_assumpt: HashMap<(Position, Counter), Instant>,

}

impl Assumptions {

    pub fn update(&mut self, p: Player, c: Position, k: Counter) {
        match p {
            Player::Existential => self.existential_assumpt.insert((c, k), Instant::now()),
            Player::Universal => self.universal_assumpt.insert((c, k), Instant::now()),
        };
    }
}

struct Justification{
    assumptions: HashSet<Position>, 
    decisions: HashSet<Position>,
}

struct Decisions{

    existential_dec: HashMap<(Position, Counter), Instant>,
    universal_dec: HashMap<(Position, Counter), Instant>,

}

impl Decisions {
    
    pub fn contains(&self, p: Player, k: &Counter, fix_types: &Vec<FixpointType>) -> bool {
        let assumpt: &HashMap<_, _> = match p {
            Player::Existential => &self.existential_dec,
            Player::Universal => &self.universal_dec,
        };

        assumpt.iter()
            .any(|((pos, kp), _)| {
                Counter::leq_p(kp, k, &p, fix_types)
            })
    }
}