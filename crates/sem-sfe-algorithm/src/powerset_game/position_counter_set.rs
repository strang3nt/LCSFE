use super::play_data::PlayData;
use super::player::Player;
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct PositionCounterSet<T> {
    eve: FxHashMap<PlayData, T>,
    adam: FxHashMap<PlayData, T>,
}

impl<T> PositionCounterSet<T> {
    pub fn default() -> PositionCounterSet<T> {
        PositionCounterSet { eve: FxHashMap::default(), adam: FxHashMap::default() }
    }

    pub fn union(&mut self, other: PositionCounterSet<T>) {
        self.eve.extend(other.eve);
        self.adam.extend(other.adam);
    }

    pub fn get_p(&self, p: &Player) -> &FxHashMap<PlayData, T> {
        match *p {
            Player::Adam => &self.adam,
            Player::Eve => &self.eve,
        }
    }

    pub fn get_mut_p(&mut self, p: &Player) -> &mut FxHashMap<PlayData, T> {
        match *p {
            Player::Adam => &mut self.adam,
            Player::Eve => &mut self.eve,
        }
    }
}
