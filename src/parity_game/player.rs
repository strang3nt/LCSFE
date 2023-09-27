#[derive(PartialEq, Eq, Clone)]
pub enum Player {
    Existential,
    Universal,
}

impl Player {
    pub fn get_opponent(p: &Player) -> Player {
        match p {
            Player::Existential => Player::Universal,
            Player::Universal => Player::Existential,
        }
    }


}
