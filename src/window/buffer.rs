use crate::codes;
use crate::constants;
use crate::logger;
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
        let initial_segment = piece_table.get_lines(0, (terminal_size.1 - 1).into());
        let stdio = Stdio::new();
        let current_line = initial_segment.get_line(1).cloned();

        let mut buffer = Buffer {
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
        };

        buffer.display_segment();
        buffer.stdio.goto_line((
            buffer.cursor.x,
            buffer.cursor.relative_y,
            buffer.cursor.absolute_y,
        ));
        buffer.stdio.cursor_block();

        buffer
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
                self.display_motion(self.cursor.vertical_x);
            }
            Motions::Up => {
                let ln = self.segment.front().unwrap().line_number;
                if usize::from(self.cursor.absolute_y) <= ln {
                    self.data.prev_line(&mut self.segment);
                    self.display_segment();
                }
                if self.cursor.absolute_y as i16 - 1 >= 1 {
                    self.cursor.move_up();
                }

                self.update_cur_line();
                self.display_motion(self.cursor.vertical_x);
            }
            Motions::Left => {
                if self.cursor.x > self.cursor.vertical_x {
                    self.cursor.set_x(self.cursor.vertical_x);
                    self.cursor.vertical_x = self.stdio.terminal_size.0;
                }

                self.cursor.move_left();
                self.display_motion(self.cursor.x);
            }
            Motions::Right => {
                if self.cursor.x > self.cursor.vertical_x {
                    self.cursor.set_x(self.cursor.vertical_x);
                    self.cursor.vertical_x = self.stdio.terminal_size.0;
                }

                let node = self.current_line.as_ref().expect("should be valid ln");
                let ln_len = self.get_ln_len(&node.value.clone());

                if self.cursor.x < ln_len {
                    self.cursor.move_right()
                }
                self.display_motion(self.cursor.x);
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
        if self.buffered_line.is_empty() {
            let ln = match &self.current_line {
                Ok(v) => v.value.clone(),
                Err(_) => String::new(),
            };

            self.buffered_line = ln.to_string();
        }

        let current_line = &self.current_line.clone().unwrap();
        let current_line_len = self.get_ln_len(&current_line.value);
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

                    // perform deletion here

                    return;
                }
                self.buffered_line.remove(usize::from(self.cursor.x - 2));

                self.stdio.update_line(&self.buffered_line, &self.cursor);
                self.motion(Motions::Left);
            }
            codes::RETURN => {
                // need to create a function that regenerates segment optimally eg only few nodes
                // 1. check if current_line == buffered_line
                // 2. if equals
                // 2.1 create new line below
                // 2.2 update buffered_line
                // 2.3 insert new line into the piece table
                // 2.4 regenerate segment
                // 2.5 update cursor
                // 2.6 return
                // 3. else
                // 3.1 calculate offset based on the current line and the buffered line
                // 3.2 create new line below
                // 3.3 insert updated line above and new line below together into the piece table
                // 3.4 regenerate segment
                // 3.5 move cursor line below
                // 3.6 return

                // no changes to the line above
                if self.cursor.x == 1 {
                    self.segment
                        .insert_at(current_line.line_number, &String::from("\n"));
                } else if self.cursor.x == current_line_len {
                    self.segment
                        .insert_at(current_line.line_number + 1, &String::from("\n"));
                } else {
                    let pt1 = &current_line.value[..(self.cursor.x - 1).into()];
                    let pt2 = &current_line.value[(self.cursor.x - 1).into()..];

                    self.segment
                        .update_at(current_line.line_number, &format!("{}{}", pt1, "\n"));
                    self.segment
                        .insert_at(current_line.line_number + 1, &String::from(pt2));

                    self.cursor.x = 1;
                    self.cursor.vertical_x = 1;
                }

                self.display_segment();

                self.motion(Motions::Down);
                return;
            }
            c => {
                let pt1 = &current_line.value[..(self.cursor.x - 1).into()];
                let pt2 = &current_line.value[(self.cursor.x - 1).into()..];

                let updated_ln = format!("{}{}{}", pt1, c, pt2);
                self.stdio.update_line(&updated_ln, &self.cursor);
                self.segment
                    .update_at(current_line.line_number, &updated_ln);
                self.update_cur_line();
                self.cursor.move_right();
            }
        }
    }

    fn update_cur_line(&mut self) {
        let new_line = self
            .segment
            .get_line(usize::from(self.cursor.absolute_y))
            .cloned()
            .unwrap();

        let new_ln_len = self.get_ln_len(&new_line.value);

        if new_ln_len < self.cursor.x {
            self.cursor.vertical_x = new_ln_len - 1;
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

    fn display_motion(&mut self, x: u16) {
        self.stdio
            .goto_line((x, self.cursor.relative_y, self.cursor.absolute_y));
    }
}
