use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_little_stone_magnitude() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D11P1 | Stone Magnitude => {}",
        contents.lines().next().unwrap()
    );
}
