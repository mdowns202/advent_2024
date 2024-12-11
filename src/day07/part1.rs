use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_calibration_total() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D07P1 | Calibration Total => {}",
        contents.lines().next().unwrap()
    );
}
