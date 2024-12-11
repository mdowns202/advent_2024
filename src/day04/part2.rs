use super::{part1::*, XMAS_FILE_PATH};

const MAS: &str = "MAS";

pub fn solve_x_mas_word_search() {
    let content = get_content(XMAS_FILE_PATH);
    let mut ws = WordSearch::new(content)
        .with_searchmode("dg")
        .with_multiplier(2)
        .with_offset(1);

    ws.find_matches(MAS);

    println!("D04P2 | 'X-MAS' Match Count => {}", ws.count);

    //ws.puzzle.iter().for_each(|ser| {
    //    println!("{:?}", ser);
    //});
}
