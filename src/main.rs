use std::env::args;
use std::io::{self, Write};
use termion::raw::IntoRawMode;

mod buffer;
mod cursor;
mod editor;

fn main() {
    let path_arg = args().nth(1);
    let filename = match path_arg {
        Some(v) => std::path::PathBuf::from(v),
        None => "".into(),
    };

    let file_result = match buffer::try_file(&filename) {
        Ok(res) => res,
        Err(_) => String::new(),
    };

    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = io::stdin();

    let mut cursor = cursor::Cursor { x: 1, y: 1 };

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
    stdout.write_all(file_result.as_bytes()).unwrap();
    stdout.flush().unwrap();

    let mut stdout = stdout.into_raw_mode().unwrap();

    let mut editor = editor::Editor::new();
    editor.run(&mut cursor, stdin, &mut stdout);
}
