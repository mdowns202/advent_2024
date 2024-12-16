use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_robot_safety_factor() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D14P1 | Robot Safety Factor => {}",
        contents.lines().next().unwrap()
    );
}
