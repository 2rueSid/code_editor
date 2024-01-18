use std::env::args;
use std::io;
use termion::raw::IntoRawMode;

mod editor;
mod files;
mod motion;

fn main() {
    let path_arg = args().nth(1);

    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = io::stdin();

    let mut editor = editor::Editor::new();
    editor.run(path_arg, stdin, &mut stdout);
}
