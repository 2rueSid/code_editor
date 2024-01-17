use std::env::args;
use std::io::{self, Write};
use termion::raw::IntoRawMode;

mod cursor;
mod editor;
mod files;

use files::buffer::Buffer;

fn main() {
    let path_arg = args().nth(1);

    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = io::stdin();

    let mut cursor = cursor::Cursor { x: 1, y: 1 };
    let buffer = Buffer::new(path_arg);

    write!(
        stdout,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(cursor.x, cursor.y),
        termion::cursor::SteadyBlock
    )
    .unwrap();
    stdout.flush().unwrap();
    stdout.suspend_raw_mode().unwrap();
    write!(stdout, "{}", buffer.data.original).unwrap();
    stdout.flush().unwrap();
    let mut stdout = stdout.into_raw_mode().unwrap();
    let mut editor = editor::Editor::new();
    editor.run(&mut cursor, stdin, &mut stdout);
}
