use super::play_data::PlayData;
use super::player::Player;
use rustc_hash::FxHashMap as HashMap;

#[derive(Debug)]
pub struct PositionCounterSet<T> {
    eve: HashMap<PlayData, T>,
    adam: HashMap<PlayData, T>,
}

impl<T> PositionCounterSet<T> {
    pub fn default() -> PositionCounterSet<T> {
        PositionCounterSet {
            eve: HashMap::default(),
            adam: HashMap::default(),
        }
    }

    pub fn union(&mut self, other: PositionCounterSet<T>) {
        self.eve.extend(other.eve);
        self.adam.extend(other.adam);
    }

    pub fn get_p(&self, p: &Player) -> &HashMap<PlayData, T> {
        match *p {
            Player::Adam => &self.adam,
            Player::Eve => &self.eve,
        }
    }

    pub fn get_mut_p(&mut self, p: &Player) -> &mut HashMap<PlayData, T> {
        match *p {
            Player::Adam => &mut self.adam,
            Player::Eve => &mut self.eve,
        }
    }

    pub fn insert(&mut self, p: &Player, k: PlayData, value: T) {
        let i = match *p {
            Player::Adam => &mut self.adam,
            Player::Eve => &mut self.eve,
        };
        if let None = i.get(&k) {
            i.insert(k, value);
        }
    }
}
