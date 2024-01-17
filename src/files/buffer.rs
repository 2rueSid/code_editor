use crate::files::cursor::Cursor;
use crate::files::piece_table::PieceTable;
use crate::motion::Motions;

use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, Read};
use termion::raw::RawTerminal;

use termion::terminal_size;
pub struct Buffer {
    file_path: std::path::PathBuf,
    pub data: PieceTable,
    pub lines: usize,
    pub cursor: Cursor,
    pub segment: VecDeque<String>,
    terminal_size: (u16, u16),
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
            cursor: Cursor { x: 1, y: 1 },
            terminal_size,
            segment: initial_segment,
        }
    }

    pub fn motion(&mut self, motion: Motions, stdout: &mut RawTerminal<io::Stdout>) {
        match motion {
            Motions::Down => self.cursor.move_down(stdout),
            Motions::Up => self.cursor.move_up(stdout),
            Motions::Left => self.cursor.move_left(stdout),
            Motions::Right => self.cursor.move_right(stdout),
        }
    }
}

fn file_content(path: &std::path::PathBuf) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;
    Ok(content)
}
