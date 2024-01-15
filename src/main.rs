use std::io::{self, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    fn move_up(&mut self) {
        if self.y > 1 {
            self.y -= 1;
        }

        print!("{}", termion::cursor::Goto(self.x, self.y));
    }

    fn move_down(&mut self) {
        self.y += 1;
        print!("{}", termion::cursor::Goto(self.x, self.y));
    }

    fn move_right(&mut self) {
        self.x += 1;

        if self.x == 90 {
            self.y += 1;
            self.x = 1;
        }
        print!("{}", termion::cursor::Goto(self.x, self.y));
    }

    fn move_left(&mut self) {
        if self.x > 1 {
            self.x -= 1;
        }

        print!("{}", termion::cursor::Goto(self.x, self.y));
    }
}

enum EditorModes {
    Normal,
    Insert,
}
struct Editor {
    mode: EditorModes,
}

impl Editor {
    fn new() -> Editor {
        Editor {
            mode: EditorModes::Normal,
        }
    }

    fn run(&mut self, cursor: &mut Cursor, stdin: io::Stdin, stdout: &mut RawTerminal<io::Stdout>) {
        writeln!(
            stdout,
            "{}{}{} q to exit.",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::SteadyBlock
        )
        .unwrap();

        stdout.flush().unwrap();
        let events = stdin.events();
        for c in events {
            let evt = c.unwrap();

            match evt {
                Event::Key(Key::Char('q')) => break,
                Event::Key(Key::Left) => cursor.move_left(),
                Event::Key(Key::Right) => cursor.move_right(),
                Event::Key(Key::Up) => cursor.move_up(),
                Event::Key(Key::Down) => cursor.move_down(),
                Event::Key(Key::Char('i')) => match self.mode {
                    EditorModes::Insert => {
                        print!("{}i", termion::cursor::SteadyBar);
                        cursor.move_right();
                    }
                    EditorModes::Normal => {
                        self.mode = EditorModes::Insert;
                        print!("{}", termion::cursor::SteadyBar)
                    }
                },
                Event::Key(Key::Esc) => {
                    self.mode = EditorModes::Normal;
                    print!("{}", termion::cursor::SteadyBlock);
                }
                _ => {}
            };

            stdout.flush().unwrap();
        }
    }
}

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = io::stdin();
    let mut cursor = Cursor { x: 1, y: 1 };

    let mut editor = Editor::new();
    editor.run(&mut cursor, stdin, &mut stdout);
}
