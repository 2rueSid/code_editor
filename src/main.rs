use std::io::{self, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

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

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut cursor = Cursor { x: 1, y: 1 };

    writeln!(
        stdout,
        "{}{}Ctrl+q to exit.",
        termion::clear::All,
        termion::cursor::Goto(cursor.x, cursor.y)
    )
    .unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Key(Key::Left) => cursor.move_left(),
            Event::Key(Key::Right) => cursor.move_right(),
            Event::Key(Key::Up) => cursor.move_up(),
            Event::Key(Key::Down) => cursor.move_down(),
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
