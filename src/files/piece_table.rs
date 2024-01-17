use std::collections::VecDeque;

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
    pub fn new(buffer: &String) -> PieceTable {
        let pieces = vec![Piece {
            source: Source::Original,
            length: buffer.len(),
            start: 0,
        }];
        PieceTable {
            original: buffer.to_string(),
            add: String::new(),
            pieces,
        }
    }

    pub fn get_lines(&self, from: usize, to: usize) -> VecDeque<String> {
        let mut current_line = 1;
        let mut res = VecDeque::with_capacity(to - from + 1 as usize);
        let mut curr_line = String::new();

        for piece in self.pieces.iter() {
            let segment = match piece.source {
                Source::Original => &self.original[piece.start..piece.start + piece.length],
                Source::Add => &self.add[piece.start..piece.start + piece.length],
            };

            for char in segment.chars() {
                if current_line >= from && to >= current_line {
                    curr_line.push(char);
                }

                if char == '\n' {
                    if current_line >= from && to >= current_line {
                        res.push_back(curr_line.clone());
                        curr_line.clear();
                    }
                    current_line += 1;
                }

                if current_line > to {
                    break;
                }
            }
        }

        if !curr_line.is_empty() && current_line >= from && current_line <= to {
            res.push_back(curr_line);
        }

        res
    }
}
