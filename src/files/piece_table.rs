enum Source {
    Original,
    Add,
}

struct Piece {
    source: Source,
    start: usize,
    length: usize,
}

pub struct PieceTable {
    pub original: String,
    pub add: String,
    pieces: Vec<Piece>,
}

impl PieceTable {
    pub fn new(buffer: String) -> PieceTable {
        let pieces = vec![Piece {
            source: Source::Original,
            length: buffer.len(),
            start: 0,
        }];
        PieceTable {
            original: buffer,
            add: String::new(),
            pieces,
        }
    }

    pub fn get_lines() {}
    pub fn edit() {}
    pub fn delete() {}
}
