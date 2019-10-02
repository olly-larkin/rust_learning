#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(ChessPosition { rank, file })
        }
    }

    pub fn in_line(&self, other: &ChessPosition) -> bool {
        self.file == other.file || self.rank == other.rank
    }

    pub fn in_diagonal(&self, other: &ChessPosition) -> bool {
        (self.file - other.file).abs() == (self.rank - other.rank).abs()
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.in_line(&other.position) || self.position.in_diagonal(&other.position)
    }
}
