#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Player {
    Eve,
    Adam,
}

impl Player {
    pub fn get_opponent(p: &Player) -> Player {
        match p {
            Player::Eve => Player::Adam,
            Player::Adam => Player::Eve,
        }
    }
}
