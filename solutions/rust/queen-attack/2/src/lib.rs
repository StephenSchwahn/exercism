#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(ChessPosition(rank, file))
        }
    }

    pub fn same_row(&self, other: &ChessPosition) -> bool {
        self.0 == other.0
    }

    pub fn same_col(&self, other: &ChessPosition) -> bool {
        self.1 == other.1
    }

    pub fn same_diagonal(&self, other: &ChessPosition) -> bool {
        // slope =(y₂ - y₁)/(x₂ - x₁)
        let slope = (self.0 - other.0, self.1 - other.1);
        if slope.0 == 0 {
            false
        } else {
            slope.1 / slope.0 == 1 || slope.1 / slope.0 == -1
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.same_row(&other.position)
            || self.position.same_col(&other.position)
            || self.position.same_diagonal(&other.position)
    }
}
