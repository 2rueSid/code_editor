use crate::window::segment::Segment;

#[derive(Clone, Debug)]
enum Source {
    Original,
    Add,
}

#[derive(Debug)]
pub struct Piece {
    source: Source,
    offset: usize,
    length: usize,
}

#[derive(Debug)]
pub struct PieceTable {
    pub original: String,
    pub add: String,
    pub pieces: Vec<Piece>,
}

impl PieceTable {
    pub fn new(buffer: &String) -> PieceTable {
        let pieces = vec![Piece {
            source: Source::Original,
            length: buffer.len(),
            offset: 0,
        }];
        PieceTable {
            original: buffer.to_string(),
            add: String::new(),
            pieces,
        }
    }

    /// create 3 pieces
    /// 1 - points to items before line
    /// 3 - poinst to items after line
    /// 2 - points to items in Add buffer
    /// append items to the Add buffer
    /// remove old piece
    /// to get offset we can store offset and length of the segment node
    /// if we have the offset we can then just make a simple calculations to get the offset
    /// parameter.
    pub fn insert(&mut self, items: &String, offset: usize) {
        if items.is_empty() {
            return;
        }

        let mut found = false;
        let mut new_pieces: Vec<Piece> = Vec::new();
        let mut found_idx = 0;

        for (i, piece) in self.pieces.iter().enumerate() {
            let is_within = piece.offset <= offset && offset < piece.offset + piece.length;

            if !is_within {
                continue;
            }

            found = true;
            found_idx = i;

            if offset > piece.offset {
                let before_piece = Piece {
                    source: Source::Original,
                    length: offset - piece.offset,
                    offset: piece.offset,
                };

                new_pieces.push(before_piece);
            }

            let new_piece = Piece {
                offset: self.add.len(),
                length: items.len(),
                source: Source::Add,
            };

            new_pieces.push(new_piece);
            self.add.push_str(&items);
            if offset < piece.offset + piece.length {
                let after_piece = Piece {
                    source: piece.source.clone(),
                    offset,
                    length: piece.offset + piece.length - offset,
                };
                new_pieces.push(after_piece);
            }

            break;
        }

        if found {
            self.pieces.splice(found_idx..=found_idx, new_pieces);
        } else if offset == self.original.len() + self.add.len() {
            let add_offset = self.add.len();
            self.add.push_str(&items);
            let new_piece = Piece {
                source: Source::Add,
                offset: add_offset,
                length: items.len(),
            };
            self.pieces.push(new_piece);
        }
    }

    /// creates a ring buffer, and iterates over pieces and it's content.
    /// in content search for the lines, if the item is equal to \n and a current line is within
    /// from-to range append it to the ring buffer.
    /// if last line is not empty and it's withing range but it may not include \n push it to the
    /// res as well.
    pub fn get_lines(&self, from: usize, to: usize) -> Segment {
        let mut current_line = 1;
        let mut res = Segment::new();
        let mut line_value = String::new();
        let mut current_offset = 0;

        for piece in self.pieces.iter() {
            let segment = match piece.source {
                Source::Original => &self.original[piece.offset..piece.offset + piece.length],
                Source::Add => &self.add[piece.offset..piece.offset + piece.length],
            };

            for char in segment.chars() {
                current_offset += char.len_utf8();
                if current_line >= from && to >= current_line {
                    line_value.push(char);
                }

                if char == '\n' {
                    if current_line >= from && to >= current_line {
                        res.new_b(line_value.clone(), current_line, current_offset);
                        line_value.clear();
                    }
                    current_line += 1;
                }

                if current_line > to {
                    break;
                }
            }
        }

        if !line_value.is_empty() && current_line >= from && current_line <= to {
            res.new_b(line_value.clone(), current_line, current_offset);
        }

        res
    }

    pub fn next_line(&self, segment: &mut Segment) {
        if let Some(last_node) = segment.back() {
            if let Some(next_line) = self
                .get_lines(last_node.line_number + 1, last_node.line_number + 1)
                .front()
                .cloned()
            {
                segment.pop_f(); // Remove the first line
                segment.add_b(next_line); // Add the new line at the end
            }
        }
    }

    pub fn prev_line(&self, segment: &mut Segment) {
        if let Some(first_node) = segment.front() {
            if first_node.line_number <= 1 {
                return;
            }
            if let Some(prev_line) = self
                .get_lines(first_node.line_number - 1, first_node.line_number - 1)
                .front()
                .cloned()
            {
                segment.pop_b();
                segment.add_f(prev_line);
            }
        }
    }

    pub fn get_string(&self) -> String {
        let mut res = String::new();
        for piece in &self.pieces {
            match piece.source {
                Source::Original => {
                    res.push_str(&self.original[piece.offset..piece.offset + piece.length]);
                }
                Source::Add => {
                    res.push_str(&self.add[piece.offset..piece.offset + piece.length]);
                }
            }
        }

        res
    }
}
