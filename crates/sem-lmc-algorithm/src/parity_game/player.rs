use std::fmt::Display;

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

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::Eve => "existantial player",
                Player::Adam => "universal player",
            }
        )
    }
}
