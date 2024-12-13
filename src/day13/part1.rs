use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_lowest_claw_tokens() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D13P1 | Lowest Claw Machine Tokens Used => {}",
        contents.lines().next().unwrap()
    );
}
