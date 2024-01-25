use std::fmt::Write as WriteFmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

pub fn log_to_file(msg: &str) {
    let t = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => {
            eprintln!("System time before UNIX EPOCH!");
            return;
        }
    };

    let mut file = match OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("logs.txt")
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Couldn't open file: {}", e);
            return;
        }
    };

    let mut formatted_msg = String::new();
    let _ = write!(formatted_msg, "\n[{}]\n{}\n\n", t, msg);

    if let Err(e) = file.write_all(formatted_msg.as_bytes()) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
