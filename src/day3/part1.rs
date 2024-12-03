use std::{fs, path::Path};

const ANSWER_FILE_PATH: &str = "src/day3/answer.txt";

pub fn print_instruction_results() {
    let file_path = Path::new(ANSWER_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    println!("D3P1 | Sum of Products => {}", contents);
}
