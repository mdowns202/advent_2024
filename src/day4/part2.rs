use super::{part1::*, TEST_FILE_PATH};

const MAS: &str = "MAS";

pub fn solve_x_mas_word_search() {
    let content = get_content(TEST_FILE_PATH);
    let mut ws = WordSearch::new(content)
        .with_searchmode("diag")
        .with_offset(1);
    ws.find_matches(MAS);
    println!("D4P2 | 'X-MAS' Match Count => {}", ws.count);
}
