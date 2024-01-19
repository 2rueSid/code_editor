use crate::motion::Motions;
use crate::window::buffer::Buffer;
use std::io::{Stdin, Write};
use termion::event::{Event, Key};

use termion::input::TermRead;
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

    pub fn run(&mut self, path: Option<String>, stdin: Stdin) {
        let mut buffer = Buffer::new(path);

        buffer.display_segment();

        for c in stdin.events() {
            let evt = c.unwrap();

            match evt {
                Event::Key(Key::Ctrl('q')) => break,
                Event::Key(Key::Left) => buffer.motion(Motions::Left),
                Event::Key(Key::Right) => buffer.motion(Motions::Right),
                Event::Key(Key::Up) => buffer.motion(Motions::Up),
                Event::Key(Key::Down) => buffer.motion(Motions::Down),
                Event::Key(Key::Char('i')) => match self.mode {
                    EditorModes::Insert => {
                        buffer.motion(Motions::Right);
                        buffer.stdio.cursor_bar();
                    }
                    EditorModes::Normal => {
                        self.mode = EditorModes::Insert;
                        buffer.stdio.cursor_bar();
                    }
                },
                Event::Key(Key::Esc) => {
                    self.mode = EditorModes::Normal;
                    buffer.stdio.cursor_block();
                }
                Event::Key(Key::Char(ch)) => {
                    if matches!(self.mode, EditorModes::Insert) {
                        buffer.edit(ch);
                    } else {
                        match ch {
                            'h' => buffer.motion(Motions::Left),
                            'l' => buffer.motion(Motions::Right),
                            'k' => buffer.motion(Motions::Up),
                            'j' => buffer.motion(Motions::Down),
                            _ => {}
                        }
                    }
                }
                _ => {}
            };
            buffer.stdio.stdout.flush().unwrap();
        }
    }
}
