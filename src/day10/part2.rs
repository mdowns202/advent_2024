use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_trail_area_rating() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D10P2 | Trail Area Rating => {}",
        contents.lines().nth(1).unwrap()
    );
}
