use std::io::{self, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::RawTerminal;

use crate::cursor::Cursor;

pub enum EditorModes {
    Normal,
    Insert,
}
pub struct Editor {
    mode: EditorModes,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            mode: EditorModes::Normal,
        }
    }

    pub fn run(
        &mut self,
        cursor: &mut Cursor,
        stdin: io::Stdin,
        stdout: &mut RawTerminal<io::Stdout>,
    ) {
        let events = stdin.events();
        for c in events {
            let evt = c.unwrap();

            match evt {
                Event::Key(Key::Ctrl('q')) => break,
                Event::Key(Key::Left) => cursor.move_left(stdout),
                Event::Key(Key::Right) => cursor.move_right(stdout),
                Event::Key(Key::Up) => cursor.move_up(stdout),
                Event::Key(Key::Down) => cursor.move_down(stdout),
                Event::Key(Key::Char('i')) => match self.mode {
                    EditorModes::Insert => {
                        write!(stdout, "{}i", termion::cursor::SteadyBar).unwrap();
                        cursor.move_right(stdout);
                    }
                    EditorModes::Normal => {
                        self.mode = EditorModes::Insert;
                        write!(stdout, "{}", termion::cursor::SteadyBar).unwrap();
                    }
                },
                Event::Key(Key::Esc) => {
                    self.mode = EditorModes::Normal;
                    write!(stdout, "{}", termion::cursor::SteadyBlock).unwrap();
                }
                Event::Key(Key::Char(ch)) => {
                    if matches!(self.mode, EditorModes::Insert) {
                        write!(stdout, "{}", ch).unwrap();
                    }
                }
                _ => {}
            };
            stdout.flush().unwrap();
        }
    }
}
