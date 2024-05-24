#[derive(Clone, Copy)]
enum Tile {
    Empty,
}

pub struct ChessBoard {
    tiles: [Tile; 64],
}

impl ChessBoard {
    pub fn new() -> Self {
        ChessBoard {
            tiles: [Tile::Empty; 64],
        }
    }
}
