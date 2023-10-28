use super::play_data::PlayData;
use super::player::Player;
use std::collections::HashMap;

#[derive(Debug)]
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
