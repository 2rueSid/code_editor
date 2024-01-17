use crate::files::piece_table::PieceTable;
use std::fs::File;
use std::io::{self, Read};

pub struct Buffer {
    file_path: std::path::PathBuf,
    pub data: PieceTable,
}

impl Buffer {
    pub fn new(path: Option<String>) -> Buffer {
        let file_path = match path {
            Some(v) => std::path::PathBuf::from(v),
            None => "".into(),
        };

        let file = match file_content(&file_path) {
            Ok(res) => res,
            Err(_) => String::new(),
        };

        let piece_table = PieceTable::new(file);

        Buffer {
            file_path,
            data: piece_table,
        }
    }

    pub fn save(&self) {}
}

fn file_content(path: &std::path::PathBuf) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;
    Ok(content)
}
