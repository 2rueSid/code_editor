use std::env::args;

mod codes;
mod constants;
mod editor;
mod logger;
mod motion;
mod stdio;
mod utils;
mod window;

fn main() {
    let path_arg = args().nth(1);
    let stdin = std::io::stdin();
    let mut editor = editor::Editor::new();
    editor.run(path_arg, stdin);
}
