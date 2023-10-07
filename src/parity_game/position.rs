use crate::parity_game::player::Player;
use std::collections::BTreeSet;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Position {
    Eve(EvePos),
    Adam(AdamPos),
}

impl Position {
    pub fn get_controller(c: &Position) -> Player {
        match c {
            Position::Eve(_) => Player::Eve,
            Position::Adam(_) => Player::Adam,
        }
    }

    /// Given a position, returns its priority.
    ///
    ///  - `priority(X) = 0`,
    ///  - `priority( (b, i) ) = i`,
    ///
    /// where X is a move from the universal player.
    pub fn priority(c: &Position) -> usize {
        match c {
            Position::Eve(EvePos { b: _, i }) => *i,
            Position::Adam(_) => 0,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]

pub struct EvePos {
    pub b: String,
    pub i: usize,
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]

pub struct AdamPos {
    pub x: Vec<BTreeSet<String>>,
}
