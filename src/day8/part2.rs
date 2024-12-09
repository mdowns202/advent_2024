use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_inline_antinode_total() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D8P2 | Inline Antinode Total => {}",
        contents.lines().nth(1).unwrap()
    );
}
