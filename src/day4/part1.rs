use std::{
    fmt::{Debug, Display},
    fs,
    path::Path,
    usize,
};

const TEST_FILE_PATH: &str = "src/day4/data/xmas.txt";
const XMAS: &str = "XMAS";

pub fn solve_xmas_word_search() {
    let content: String = get_content(TEST_FILE_PATH);
    let mut ws = WordSearch::new(content);
    ws.find_matches(XMAS);

    println!("\nXMAS Count: {}", ws.count);
}

pub fn get_content(path_str: &str) -> String {
    let file_path = Path::new(path_str);
    let content: String =
        fs::read_to_string(file_path).expect("file not found or could not be read");

    content
}

#[derive(Debug)]
struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    fn point(&mut self, x: usize, y: usize) -> &mut Self {
        (self.x, self.y) = (x, y);
        self
    }
}

#[derive(Debug)]
struct SearchPuzzle<T> {
    puzzle: Vec<Vec<T>>,
    cursor: Cursor,
    count: u16,
}

impl<T: Clone + Debug> SearchPuzzle<T>
where
    Vec<T>: FromIterator<char>,
    T: PartialEq<char>,
    T: Display,
{
    fn new(content: String) -> Self {
        let rows: Vec<&str> = content.lines().collect();
        let puzzle: Vec<Vec<T>> = rows.iter().map(|row| row.chars().collect()).collect();
        let cursor = Cursor { x: 0, y: 0 };
        Self {
            puzzle,
            cursor,
            count: 0,
        }
    }

    fn find_matches(&mut self, word: &str) {
        let puzzle = self.puzzle.clone();
        let characters = word.chars().collect::<Vec<char>>();
        puzzle.iter().enumerate().for_each(|(x, row)| {
            row.iter().enumerate().for_each(|(y, letter)| {
                if *letter == characters[0] {
                    self.check_for_match(x, y)
                }
            });
        });
    }
    fn check_for_match(&mut self, x: usize, y: usize) {
        self.cursor.point(x, y);
        self.cursor_sweep();
    }

    // Put all 4 letter combos in Vec and match &str
    fn cursor_sweep(&mut self) {
        let default = &Vec::default();
        let mut possibilities: Vec<Vec<_>> = Vec::new();
        let (x, y) = (self.cursor.x, self.cursor.y);
        let puzzle = &self.puzzle;
        let row = puzzle[x].clone();
        let current = row.get(y);

        let forward = vec![current, row.get(y + 1), row.get(y + 2), row.get(y + 3)];
        possibilities.push(forward);

        let backward = vec![
            current,
            row.get(y.checked_sub(1).unwrap_or(usize::MAX)),
            row.get(y.checked_sub(2).unwrap_or(usize::MAX)),
            row.get(y.checked_sub(3).unwrap_or(usize::MAX)),
        ];
        possibilities.push(backward);

        let upward = vec![
            current,
            puzzle
                .get(x.checked_sub(1).unwrap_or(usize::MAX))
                .unwrap_or(default)
                .get(y),
            puzzle
                .get(x.checked_sub(2).unwrap_or(usize::MAX))
                .unwrap_or(default)
                .get(y),
            puzzle
                .get(x.checked_sub(3).unwrap_or(usize::MAX))
                .unwrap_or(default)
                .get(y),
        ];
        possibilities.push(upward);

        let downward = vec![
            current,
            puzzle.get(x + 1).unwrap_or(default).get(y),
            puzzle.get(x + 2).unwrap_or(default).get(y),
            puzzle.get(x + 3).unwrap_or(default).get(y),
        ];
        possibilities.push(downward);

        let upright = vec![
            current,
            puzzle
                .get(x.checked_sub(1).unwrap_or(usize::MAX))
                .unwrap_or(default)
                .get(y + 1),
            puzzle
                .get(x.checked_sub(2).unwrap_or(usize::MAX))
                .unwrap_or(default)
                .get(y + 2),
            puzzle
                .get(x.checked_sub(3).unwrap_or(usize::MAX))
                .unwrap_or(default)
                .get(y + 3),
        ];
        possibilities.push(upright);

        let downright = vec![
            current,
            puzzle.get(x + 1).unwrap_or(default).get(y + 1),
            puzzle.get(x + 2).unwrap_or(default).get(y + 2),
            puzzle.get(x + 3).unwrap_or(default).get(y + 3),
        ];
        possibilities.push(downright);

        let upleft = vec![
            current,
            puzzle
                .get(x.checked_sub(1).unwrap_or(usize::MAX))
                .unwrap_or(default)
                .get(y.checked_sub(1).unwrap_or(usize::MAX)),
            puzzle
                .get(x.checked_sub(2).unwrap_or(usize::MAX))
                .unwrap_or(default)
                .get(y.checked_sub(2).unwrap_or(usize::MAX)),
            puzzle
                .get(x.checked_sub(3).unwrap_or(usize::MAX))
                .unwrap_or(default)
                .get(y.checked_sub(3).unwrap_or(usize::MAX)),
        ];
        possibilities.push(upleft);

        let downleft = vec![
            current,
            puzzle
                .get(x + 1)
                .unwrap_or(default)
                .get(y.checked_sub(1).unwrap_or(usize::MAX)),
            puzzle
                .get(x + 2)
                .unwrap_or(default)
                .get(y.checked_sub(2).unwrap_or(usize::MAX)),
            puzzle
                .get(x + 3)
                .unwrap_or(default)
                .get(y.checked_sub(3).unwrap_or(usize::MAX)),
        ];
        possibilities.push(downleft);

        possibilities.iter().for_each(|poss| {
            let mut possible_match = String::new();
            poss.iter().for_each(|letter| {
                match letter {
                    Some(l) => possible_match.push_str(&l.to_string()),
                    None => (),
                };
            });
            match possible_match {
                m if &m == XMAS => {
                    println!("{}", m);
                    self.count += 1;
                }
                _ => (),
            }
        });
    }
}

type WordSearch = SearchPuzzle<char>;

