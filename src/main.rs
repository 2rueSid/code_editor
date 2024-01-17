use files::buffer::Buffer;
use std::env::args;
use std::io::{self, Write};
use termion::raw::IntoRawMode;

mod editor;
mod files;
mod motion;

fn main() {
    let path_arg = args().nth(1);

    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = io::stdin();

    let mut buffer = Buffer::new(path_arg);

    write!(stdout, "{}", termion::clear::All,).unwrap();
    stdout.flush().unwrap();

    let mut editor = editor::Editor::new();
    editor.run(&mut buffer, stdin, &mut stdout);
}
