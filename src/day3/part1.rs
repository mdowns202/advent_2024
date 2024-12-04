use super::ANSWER_FILE_PATH;
use std::{fs, path::Path};

pub fn print_reg_instruction_prod_sum() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!(
        "D3P1 | Sum of Products => {}",
        contents.lines().next().unwrap()
    );
}
