use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_cond_instruction_prod_sum() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D03P2 | Conditional Product Sum => {}",
        contents.lines().nth(1).unwrap()
    );
}
