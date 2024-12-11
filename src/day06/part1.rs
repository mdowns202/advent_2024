use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_median_safety_sum() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D05P1 | Median Safety Sum => {}",
        contents.lines().next().unwrap()
    );
}
