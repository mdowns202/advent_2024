use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_lanternfish_gps_sum() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D15P1 | Lanternfish GPS Sum => {}",
        contents.lines().next().unwrap()
    );
}
