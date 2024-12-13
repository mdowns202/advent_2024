use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_fence_price() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D12P1 | Fence Price => {}",
        contents.lines().next().unwrap()
    );
}
