struct Playlist(Vec<(Position, Counter, HashSet<UnexploredMoves>)>);

impl Playlist {

    pub fn contains_position(&self, c: &Position, k: &Counter) -> Option<Counter> {
        unimplemented!()
    }

    pub fn push(&mut self, p: Position, k: Counter, pi: HashSet<UnexploredMoves>) {
        self.0.push((p, k, pi));
    }

    pub fn isEmpty(&self) -> bool {
        self.0.is_empty()
    }
}