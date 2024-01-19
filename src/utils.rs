use std::fs::File;
use std::io::{Error, Read};

pub fn file_content(path: &std::path::PathBuf) -> Result<String, Error> {
    let mut f = File::open(path)?;
    let mut content = String::new();

    f.read_to_string(&mut content)?;
    Ok(content)
}
