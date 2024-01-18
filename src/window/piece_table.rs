use crate::window::buffer::SegmentNode;
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

    pub fn delete(&self) {}
    pub fn insert(&self) {}
    pub fn item_at(&self) {}

    pub fn get_lines(&self, from: usize, to: usize) -> VecDeque<SegmentNode> {
        let mut current_line = 1;
        let mut res = VecDeque::with_capacity(to - from + 1 as usize);
        let mut line_value = String::new();

        for piece in self.pieces.iter() {
            let segment = match piece.source {
                Source::Original => &self.original[piece.start..piece.start + piece.length],
                Source::Add => &self.add[piece.start..piece.start + piece.length],
            };

            for char in segment.chars() {
                if current_line >= from && to >= current_line {
                    line_value.push(char);
                }

                if char == '\n' {
                    if current_line >= from && to >= current_line {
                        res.push_back(SegmentNode {
                            value: line_value.clone(),
                            line_number: current_line,
                        });
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
            res.push_back(SegmentNode {
                value: line_value.clone(),
                line_number: current_line,
            });
        }

        res
    }

    pub fn next_line(&self, segment: &mut VecDeque<SegmentNode>) {
        if let Some(last_node) = segment.back() {
            if let Some(next_line) = self
                .get_lines(last_node.line_number + 1, last_node.line_number + 1)
                .front()
                .cloned()
            {
                segment.pop_front(); // Remove the first line
                segment.push_back(next_line); // Add the new line at the end
            }
        }
    }

    pub fn prev_line(&self, segment: &mut VecDeque<SegmentNode>) {
        if let Some(first_node) = segment.front() {
            if first_node.line_number <= 1 {
                return;
            }
            if let Some(prev_line) = self
                .get_lines(first_node.line_number - 1, first_node.line_number - 1)
                .front()
                .cloned()
            {
                segment.pop_back();
                segment.push_front(prev_line);
            }
        }
    }
}
