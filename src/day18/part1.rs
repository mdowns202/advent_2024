use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_chrono_cpu_output() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D17P1 | Chrono CPU Output => {}",
        contents.lines().next().unwrap()
    );
}
