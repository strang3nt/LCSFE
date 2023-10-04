use crate::parity_game::player::Player;
use std::collections::BTreeSet;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Position {
    Eve(String, usize),
    Adam(Vec<BTreeSet<String>>),
}

impl Position {
    pub fn get_controller(c: &Position) -> Player {
        match c {
            Position::Eve(_, _) => Player::Eve,
            Position::Adam(_) => Player::Adam,
        }
    }

    pub fn priority(c: &Position) -> &usize {
        match c {
            Position::Eve(_, i) => i,
            Position::Adam(_) => &0,
        }
    }
}
