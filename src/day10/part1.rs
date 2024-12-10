use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_total_trailhead_score() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D10P1 | Total Trailhead Score => {}",
        contents.lines().next().unwrap()
    );
}
