#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank < 0 || rank > 7 || file < 0 || file > 7 {
            None
        } else {
            Some(ChessPosition { rank: rank, file: file })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.file == other.position.file || 
        self.position.rank == other.position.rank || 
        self.can_use_diagonal(other)
    }

    fn can_use_diagonal(&self, other: &Queen) -> bool {
        (self.position.file - other.position.file).abs() == (self.position.rank - other.position.rank).abs()
    }
}
