use crate::parity_game::play_data::PlayData;
use crate::parity_game::player::Player;
use crate::parity_game::position::Position;
use std::collections::{HashMap, HashSet};

use super::position::{AdamPos, EvePos};

pub struct PositionCounterSet<T> {
    eve: HashMap<PlayData, T>,
    adam: HashMap<PlayData, T>,
}

impl<T> PositionCounterSet<T> {
    pub fn new() -> PositionCounterSet<T> {
        PositionCounterSet { eve: HashMap::new(), adam: HashMap::new() }
    }

    pub fn get_p(&self, p: &Player) -> &HashMap<PlayData, T> {
        match p {
            &Player::Adam => &self.adam,
            &Player::Eve => &self.eve,
        }
    }

    pub fn get_mut_p(&mut self, p: &Player) -> &mut HashMap<PlayData, T> {
        match p {
            &Player::Adam => &mut self.adam,
            &Player::Eve => &mut self.eve,
        }
    }
}

pub enum Justification {
    Truth,
    SetOfMoves(HashSet<Position>),
}
