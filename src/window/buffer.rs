use crate::motion::Motions;
use crate::window::cursor::Cursor;
use crate::window::piece_table::PieceTable;
use crate::window::segment::Segment;

use std::fs::File;
use std::io::{self, Read, Write};
use termion::clear;
use termion::raw::RawTerminal;

use termion::terminal_size;

pub struct Buffer {
    file_path: std::path::PathBuf,
    pub data: PieceTable,
    pub lines: usize,
    pub cursor: Cursor,
    pub segment: Segment,
    terminal_size: (u16, u16),
    char_items: Vec<char>,
}

impl Buffer {
    pub fn new(path: Option<String>) -> Buffer {
        let file_path = match path {
            Some(v) => std::path::PathBuf::from(v),
            None => "".into(),
        };

        let file = match file_content(&file_path) {
            Ok(res) => res,
            Err(_) => String::new(),
        };

        let terminal_size = terminal_size().unwrap();
        let piece_table = PieceTable::new(&file);
        let initial_segment = piece_table.get_lines(0, terminal_size.1.into());

        Buffer {
            file_path,
            data: piece_table,
            lines: file.lines().count().clone(),
            cursor: Cursor {
                x: 1,
                relative_y: 1,
                absolute_y: 1,
            },
            terminal_size,
            segment: initial_segment,
            char_items: Vec::new(),
        }
    }

    pub fn display_segment(&self, stdout: &mut RawTerminal<io::Stdout>) {
        let lines = self.segment.construct_segment();
        stdout.suspend_raw_mode().unwrap();
        write!(stdout, "{}{}", clear::All, lines,).unwrap();

        stdout.flush().unwrap();

        stdout.activate_raw_mode().unwrap();
    }

    pub fn motion(&mut self, motion: Motions, stdout: &mut RawTerminal<io::Stdout>) {
        match motion {
            Motions::Down => {
                if self.cursor.relative_y + 1 >= self.terminal_size.1 {
                    self.data.next_line(&mut self.segment);
                    self.display_segment(stdout);
                }
                self.cursor.move_down(self.terminal_size.1);
            }
            Motions::Up => {
                let ln = self.segment.front().unwrap().line_number;
                if usize::from(self.cursor.absolute_y) < ln {
                    self.data.prev_line(&mut self.segment);
                    self.display_segment(stdout);
                }
                if self.cursor.absolute_y as i16 - 1 >= 1 {
                    self.cursor.move_up();
                }
            }
            Motions::Left => self.cursor.move_left(),
            Motions::Right => self.cursor.move_right(),
        }

        self.display_cursor(stdout);
    }

    pub fn edit(&mut self, item: char, stdout: &mut RawTerminal<io::Stdout>) {
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
        let node = self
            .segment
            .get_line(usize::from(self.cursor.absolute_y + 1))
            .expect("Line not valid under the curosr");

        self.debug_print(stdout, node, 2);
        self.debug_print(stdout, self.cursor.absolute_y, 3);
    }

    fn display_cursor(&self, stdout: &mut RawTerminal<io::Stdout>) {
        let cursor_position_str = format!("x: {} y: {}", self.cursor.x, self.cursor.absolute_y);
        let offset = cursor_position_str.len();
        let x = self.terminal_size.0 - offset as u16;
        let y = self.terminal_size.1 - 1;
        stdout.suspend_raw_mode().unwrap();
        write!(
            stdout,
            "{}{}{}{}",
            termion::cursor::Goto(x, y),
            clear::CurrentLine,
            cursor_position_str,
            termion::cursor::Goto(self.cursor.x, self.cursor.relative_y)
        )
        .unwrap();
        stdout.flush().unwrap();
        stdout.activate_raw_mode().unwrap();
    }

    fn debug_print<T: std::fmt::Debug>(
        &self,
        stdout: &mut RawTerminal<io::Stdout>,
        data: T,
        y_offset: u16,
    ) {
        let str = format!("{:?}", data);
        let x = 1;
        let y = self.terminal_size.1 as i16 - y_offset as i16;
        stdout.suspend_raw_mode().unwrap();
        write!(
            stdout,
            "{}{}{}{}",
            termion::cursor::Goto(x, y as u16),
            clear::CurrentLine,
            str,
            termion::cursor::Goto(self.cursor.x, self.cursor.relative_y)
        )
        .unwrap();
        stdout.flush().unwrap();
        stdout.activate_raw_mode().unwrap();
    }
}

fn file_content(path: &std::path::PathBuf) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;
    Ok(content)
}
