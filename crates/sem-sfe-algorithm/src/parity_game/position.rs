use super::player::Player;
use std::{collections::BTreeSet, fmt::Display};

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

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Position::Adam(x) => format!("{}", x),
                Position::Eve(x) => format!("{}", x),
            }
        )
    }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]

pub struct EvePos {
    pub b: String,
    pub i: usize,
}

impl Display for EvePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.b, self.i)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]

pub struct AdamPos {
    pub x: Vec<BTreeSet<String>>,
}

impl Display for AdamPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({})",
            self.x
                .iter()
                .map(|x_i| format!("{:?}", x_i))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
