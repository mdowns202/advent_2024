use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_filesystem_checksum_value() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D09P1 | Checksum Value => {}",
        contents.lines().next().unwrap()
    );
}
