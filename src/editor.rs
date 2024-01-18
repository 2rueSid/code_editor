use crate::motion::Motions;
use crate::window::buffer::Buffer;
use std::io::{self, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::RawTerminal;

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
        path: Option<String>,
        stdin: io::Stdin,
        stdout: &mut RawTerminal<io::Stdout>,
    ) {
        let events = stdin.events();
        let mut buffer = Buffer::new(path);
        buffer.display_segment(stdout);

        for c in events {
            let evt = c.unwrap();

            match evt {
                Event::Key(Key::Ctrl('q')) => break,
                Event::Key(Key::Left) => buffer.motion(Motions::Left, stdout),
                Event::Key(Key::Right) => buffer.motion(Motions::Right, stdout),
                Event::Key(Key::Up) => buffer.motion(Motions::Up, stdout),
                Event::Key(Key::Down) => buffer.motion(Motions::Down, stdout),
                Event::Key(Key::Char('i')) => match self.mode {
                    EditorModes::Insert => {
                        write!(stdout, "{}i", termion::cursor::SteadyBar).unwrap();
                        buffer.motion(Motions::Right, stdout);
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
                    } else {
                        match ch {
                            'h' => buffer.motion(Motions::Left, stdout),
                            'l' => buffer.motion(Motions::Right, stdout),
                            'k' => buffer.motion(Motions::Up, stdout),
                            'j' => buffer.motion(Motions::Down, stdout),
                            _ => {}
                        }
                    }
                }
                _ => {}
            };
            stdout.flush().unwrap();
        }
    }
}
