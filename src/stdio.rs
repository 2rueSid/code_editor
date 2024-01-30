use crate::window::cursor::Cursor;
use std::io::{self, Stdout, Write};
use termion::clear;

use termion::raw::{IntoRawMode, RawTerminal};

pub struct Stdio {
    pub stdout: RawTerminal<Stdout>,
    pub terminal_size: (u16, u16),
}

impl Stdio {
    pub fn new() -> Stdio {
        let stdout = io::stdout().into_raw_mode().unwrap();

        Stdio {
            stdout,
            terminal_size: termion::terminal_size().unwrap(),
        }
    }

    pub fn cursor_block(&mut self) {
        write!(self.stdout, "{}", termion::cursor::SteadyBlock).unwrap();
        self.stdout.flush().unwrap();
    }
    pub fn cursor_bar(&mut self) {
        write!(self.stdout, "{}", termion::cursor::SteadyBar).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn display_below(&mut self, x: u16, y: u16, data: &String) {
        self.stdout.suspend_raw_mode().unwrap();
        write!(
            self.stdout,
            "{}{}{}",
            termion::cursor::Goto(x, y),
            termion::clear::AfterCursor,
            data
        )
        .unwrap();
        self.stdout.flush().unwrap();

        self.stdout.activate_raw_mode().unwrap();
    }

    pub fn goto_line(&mut self, pos: (u16, u16, u16)) {
        self.display_cursor(pos.0, pos.2);
        self.goto(pos.0, pos.1);
    }

    pub fn update_line(&mut self, line: &String, c: &Cursor) {
        write!(
            self.stdout,
            "{}{}{}{}",
            termion::cursor::Goto(1, c.relative_y),
            clear::CurrentLine,
            line,
            termion::cursor::Goto(c.x + 1, c.relative_y)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }
    pub fn update_line_at(&mut self, line: &String, c: (u16, u16)) {
        write!(
            self.stdout,
            "{}{}{}{}",
            termion::cursor::Goto(1, c.1),
            clear::CurrentLine,
            line,
            termion::cursor::Goto(c.0 + 1, c.1)
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn display_segment(&mut self, text: String, c: (u16, u16)) {
        self.stdout.suspend_raw_mode().unwrap();
        write!(
            self.stdout,
            "{}{}{}{}",
            termion::cursor::Goto(1, 1),
            clear::All,
            text,
            termion::cursor::Goto(c.0, c.1)
        )
        .unwrap();

        self.stdout.flush().unwrap();

        self.stdout.activate_raw_mode().unwrap();
    }

    fn goto(&mut self, x: u16, y: u16) {
        write!(self.stdout, "{}", termion::cursor::Goto(x, y)).unwrap();
        self.stdout.flush().unwrap();
    }

    fn display_cursor(&mut self, x: u16, abs_y: u16) {
        let cursor_position_str = format!("x: {} y: {}", x, abs_y);

        let offset = cursor_position_str.len();
        let x = self.terminal_size.0 - offset as u16;
        let y = self.terminal_size.1;
        write!(
            self.stdout,
            "{}{}{}",
            termion::cursor::Goto(x, y),
            clear::CurrentLine,
            cursor_position_str,
        )
        .unwrap();
        self.stdout.flush().unwrap();
    }
}
