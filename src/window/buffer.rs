use crate::codes;
use crate::constants;
use crate::motion::Motions;
use crate::stdio::Stdio;
use crate::utils;
use crate::window::cursor::Cursor;
use crate::window::piece_table::PieceTable;
use crate::window::segment::{Segment, SegmentNode};

use termion::terminal_size;

pub struct Buffer {
    pub data: PieceTable,
    pub stdio: Stdio,
    pub lines: usize,
    pub cursor: Cursor,
    pub segment: Segment,
    current_line: Result<SegmentNode, String>,
    char_items: Vec<char>,
    buffered_line: String,
    file_path: std::path::PathBuf,
}

impl Buffer {
    pub fn new(path: Option<String>) -> Buffer {
        let file_path = match path {
            Some(v) => std::path::PathBuf::from(v),
            None => "".into(),
        };

        let file = match utils::file_content(&file_path) {
            Ok(res) => res,
            Err(_) => String::new(),
        };

        let terminal_size = terminal_size().unwrap();
        let piece_table = PieceTable::new(&file);
        let initial_segment = piece_table.get_lines(0, terminal_size.1.into());
        let stdio = Stdio::new();
        let current_line = initial_segment.get_line(1).cloned();

        Buffer {
            file_path,
            data: piece_table,
            lines: file.lines().count().clone(),
            cursor: Cursor {
                x: 1,
                vertical_x: 1,
                relative_y: 1,
                absolute_y: 1,
            },
            segment: initial_segment,
            stdio,
            char_items: Vec::new(),
            buffered_line: String::new(),
            current_line,
        }
    }

    pub fn display_segment(&mut self) {
        let text = self.segment.construct_segment();

        self.stdio.display_segment(text);
    }

    pub fn motion(&mut self, motion: Motions) {
        match motion {
            Motions::Down => {
                if self.cursor.relative_y + 1 >= self.stdio.terminal_size.1 {
                    self.data.next_line(&mut self.segment);
                    self.display_segment();
                }
                self.cursor.move_down(self.stdio.terminal_size.1);
                self.update_cur_line();
                self.stdio
                    .goto(self.cursor.vertical_x, self.cursor.relative_y);
            }
            Motions::Up => {
                let ln = self.segment.front().unwrap().line_number;
                if usize::from(self.cursor.absolute_y) < ln {
                    self.data.prev_line(&mut self.segment);
                    self.display_segment();
                }
                if self.cursor.absolute_y as i16 - 1 >= 1 {
                    self.cursor.move_up();
                }

                self.update_cur_line();
                self.stdio
                    .goto(self.cursor.vertical_x, self.cursor.relative_y);
            }
            Motions::Left => {
                self.cursor.move_left();

                self.stdio.display_cursor(&self.cursor);
            }
            Motions::Right => {
                let node = self.current_line.as_ref().expect("should be valid ln");
                let ln_len = self.get_ln_len(&node.value.clone());
                if self.cursor.x + 1 <= ln_len {
                    self.cursor.move_right()
                }

                self.stdio.display_cursor(&self.cursor);
            }
        }
    }

    pub fn edit(&mut self, item: char) {
        // we have relative x and absolute y;
        //
        // we need to go to the ring buffer and find exact line where we currently at
        // each segment node in the buffer is one line in the editor
        // so we can get the absolute_y iterate over buffer and find the exact line where we
        // currently at.
        //
        // each segment node have also an offset fields, which should point to the place where the
        // line starts. so in order to find the exact place we need to add x to line offset
        //
        // to handle inserts, we will use lines for now, as it seems like less complicated and will
        // be slightly more efficient. but potential bottleneck for this approach are small one
        // character updates, which will led to saving whole line.
        //
        // or we can do both actually. keep tracking buffer of character that are typed and if
        // changes are made in one place, eg without jumping in many places in the line.
        // and we can also track a buffered line that will contain all updates to efficently redraw
        // it. and if it changed in many places we will need to insert the whole line, otherwise,
        // insert only buffered sequence of characters.
        //
        // to display stdout efficently, we may consider to get current cursor, remove current line
        // and write only buffered line. so buffered line should keep all changed that we typed,
        // and we also need to keep character sequence to later determine what we should insert
        // into the piece table.
        //
        // both variants should react to \n to make an insertion.
        // when we handle a new line we need to write it as well, so when new line is created, we
        // need to shift front item from the buffer, and rearange buffer,
        let node = self
            .segment
            .get_line(usize::from(self.cursor.absolute_y + 1))
            .expect("Line not valid under the curosr");

        self.stdio.debug_print(node, 2, &self.cursor);
        self.stdio
            .debug_print(self.cursor.absolute_y, 3, &self.cursor);

        if self.buffered_line.is_empty() {
            self.buffered_line = node.value.clone();
        }

        // handle changes in line
        match item {
            codes::BACKSPACE => {
                if self.cursor.absolute_y == 1 && self.cursor.x == 1 {
                    return;
                }

                if self.cursor.relative_y == 1 && self.cursor.x == 1 {
                    // fetch new line here
                    return;
                }

                if (self.cursor.x as i16 - 2) < 0 {
                    // if x is less than 0, merge with line y - 1
                    let prev_line = match self.segment.get_line(usize::from(self.cursor.absolute_y))
                    {
                        Ok(v) => v,
                        Err(_) => return,
                    };

                    let mut curr_line = self.buffered_line.clone();
                    let mut pl_value = prev_line.value.clone();

                    pl_value.pop();
                    curr_line.pop();
                    self.stdio.update_line(&String::from("\n"), &self.cursor);

                    self.cursor.x = self.get_ln_len(&pl_value);
                    self.cursor.move_up();

                    self.buffered_line = format!("{}{}\n", pl_value, curr_line);

                    self.stdio.update_line(&self.buffered_line, &self.cursor);
                    return;
                }
                self.buffered_line.remove(usize::from(self.cursor.x - 2));
                self.stdio.debug_print(&self.buffered_line, 4, &self.cursor);

                self.stdio.update_line(&self.buffered_line, &self.cursor);
                self.motion(Motions::Left);
            }
            c => {
                let (first_pt, second_pt) = self.split_bl(usize::from(self.cursor.x - 1));
                self.buffered_line = format!("{}{}{}", first_pt, c, second_pt);
                self.stdio.debug_print(&self.buffered_line, 4, &self.cursor);

                self.stdio.update_line(&self.buffered_line, &self.cursor);
                self.motion(Motions::Right);
            }
        }
    }

    fn split_bl(&self, i: usize) -> (&str, &str) {
        let first_pt = &self.buffered_line[..i];
        let second_pt = &self.buffered_line[i..];

        (first_pt, second_pt)
    }

    fn update_cur_line(&mut self) {
        let new_line = self
            .segment
            .get_line(usize::from(self.cursor.absolute_y + 1))
            .cloned()
            .expect("line should exist");

        let new_ln_len = self.get_ln_len(&new_line.value);

        if new_ln_len < self.cursor.x {
            self.cursor.vertical_x = new_ln_len;
        } else {
            self.cursor.vertical_x = self.cursor.x;
        }

        self.current_line = Ok(new_line);
    }

    fn get_ln_len(&mut self, ln: &String) -> u16 {
        let mut node_len = ln.len() as u16;
        let tabs = ln.matches("\t").count();

        if tabs > 0 {
            node_len += tabs as u16 * constants::TABULATION_COUNT;
        }

        node_len
    }
}
