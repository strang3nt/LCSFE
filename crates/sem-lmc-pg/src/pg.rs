#[derive(Debug)]
pub struct PG(pub Vec<(Node, Vec<u32>)>);

#[derive(Debug)]

pub struct Node {
    pub id: u32,
    pub owner: Player, //
    pub parity: u32,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Player {
    Adam,
    Eve,
}
