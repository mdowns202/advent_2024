use super::{part1::*, TEST_FILE_PATH};

const MAS: &str = "MAS";

pub fn solve_x_mas_word_search() {
    let content = get_content(TEST_FILE_PATH);
    let mut ws = WordSearch::new(content);
    ws.with_searchmode("diag");
}
