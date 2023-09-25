use crate::parity_game::player::Player;
use std::collections::BTreeSet;

#[derive(PartialEq, Eq, Hash)]
pub enum Position{
    Existential(String, usize),
    Universal(Vec<BTreeSet<String>>),
}

impl Position {

    pub fn get_controller(c: &Position) -> Player {
        match c {
            Position::Existential(_, _) => Player::Existential,
            Position::Universal(_) => Player::Universal,
        }
    }
}