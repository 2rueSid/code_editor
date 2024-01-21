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

    pub fn goto(&mut self, x: u16, y: u16) {
        write!(self.stdout, "{}", termion::cursor::Goto(x, y)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn display_cursor(&mut self, c: &Cursor) {
        let cursor_position_str = format!("x: {} y: {}", c.x, c.absolute_y);

        let offset = cursor_position_str.len();
        let x = self.terminal_size.0 - offset as u16;
        let y = self.terminal_size.1 - 1;
        self.stdout.suspend_raw_mode().unwrap();
        write!(
            self.stdout,
            "{}{}{}{}",
            termion::cursor::Goto(x, y),
            clear::CurrentLine,
            cursor_position_str,
            termion::cursor::Goto(c.x, c.relative_y)
        )
        .unwrap();
        self.stdout.flush().unwrap();
        self.stdout.activate_raw_mode().unwrap();
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

    pub fn debug_print<T: std::fmt::Debug>(&mut self, data: T, y_offset: u16, c: &Cursor) {
        let str = format!("{:?}", data);
        let x = 1;
        let y = self.terminal_size.1 as i16 - y_offset as i16;
        self.stdout.suspend_raw_mode().unwrap();
        write!(
            self.stdout,
            "{}{}{}{}",
            termion::cursor::Goto(x, y as u16),
            clear::CurrentLine,
            str,
            termion::cursor::Goto(c.x, c.relative_y)
        )
        .unwrap();
        self.stdout.flush().unwrap();
        self.stdout.activate_raw_mode().unwrap();
    }

    pub fn display_segment(&mut self, text: String) {
        self.stdout.suspend_raw_mode().unwrap();
        write!(self.stdout, "{}{}", clear::All, text).unwrap();

        self.stdout.flush().unwrap();

        self.stdout.activate_raw_mode().unwrap();
    }
}
