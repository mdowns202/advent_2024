use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_antinode_total() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D08P1 | Antinode Total => {}",
        contents.lines().next().unwrap()
    );
}
