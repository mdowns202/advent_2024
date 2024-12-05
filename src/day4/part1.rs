use std::{fmt::Debug, fs, path::Path};

const TEST_FILE_PATH: &str = "src/day4/data/test.txt";
const XMAS: &str = "XMAS";

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
struct SearchPuzzle<T> {
    puzzle: Vec<Vec<T>>,
    cursor: Cursor,
    count: u16,
}

impl<T: Clone + Debug> SearchPuzzle<T>
where
    Vec<T>: FromIterator<char>,
    T: PartialEq<char>,
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
            println!("{:?}", row);
            row.iter().enumerate().for_each(|(y, letter)| {
                if *letter == characters[0] {
                    if self.check_for_match(x, y) {
                        self.count += 1;
                    };
                }
            });
        });
    }
    fn check_for_match(&mut self, x: usize, y: usize) -> bool {
        let possible_match = true;
        self.cursor.point(x, y);

        self.identify_rows(self.cursor.x);
        self.identify_cols(self.cursor.y);

        self.cursor_sweep();

        possible_match
    }
    fn identify_rows(&self, i: usize) -> Vec<Vec<T>> {
        let mut row_iter = self.puzzle.iter();
        let current_row: Vec<T> = row_iter.nth(i).unwrap().to_vec();
        let mut next_row: Vec<T> = Vec::new();
        let mut prev_row: Vec<T> = Vec::new();
        let x_len = self.puzzle.len();

        match i {
            0 => next_row = row_iter.next().unwrap().to_vec(),
            x if x >= x_len - 1 => prev_row = self.puzzle.get(i - 1).unwrap().to_vec(),
            _ => {
                next_row = row_iter.next().unwrap().to_vec();
                prev_row = self.puzzle.get(i - 1).unwrap().to_vec();
            }
        };
        vec![current_row, next_row, prev_row]
    }

    fn identify_cols(&self, j: usize) -> Vec<Vec<&T>> {
        let current_col: Vec<&T> = self.puzzle.iter().map(|ser| ser.get(j).unwrap()).collect();
        let next_col: Option<Vec<&T>> = self
            .puzzle
            .iter()
            .map(|ser| {
                if j <= ser.len() - 1 {
                    ser.get(j + 1)
                } else {
                    None
                }
            })
            .collect();
        let prev_col: Option<Vec<&T>> = self
            .puzzle
            .iter()
            .map(|ser| if j > 0 { ser.get(j - 1) } else { None })
            .collect();

        vec![
            current_col,
            prev_col.unwrap_or(Vec::new()),
            next_col.unwrap_or(Vec::new()),
        ]
    }

    fn cursor_sweep(&self) {}
}

type WordSearch = SearchPuzzle<char>;

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

