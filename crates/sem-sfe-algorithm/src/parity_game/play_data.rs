use std::rc::Rc;

use super::position::Position;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct PlayData {
    pub pos: Position,
    pub k: Rc<Vec<u32>>,
}
