use super::XMAS_FILE_PATH;
use std::{fmt::Debug, fs, path::Path, usize};

const XMAS: &str = "XMAS";

pub fn solve_xmas_word_search() {
    let content: String = get_content(XMAS_FILE_PATH);
    let mut ws = WordSearch::new(content);
    ws.find_matches(XMAS);
    println!("D4P1 | 'XMAS' Match Count => {}", ws.count);
}

pub fn get_content(path_str: &str) -> String {
    let file_path = Path::new(path_str);
    let content: String =
        fs::read_to_string(file_path).expect("file not found or could not be read");

    content
}

#[derive(Debug)]
pub struct SearchPuzzle<T> {
    puzzle: Vec<Vec<T>>,
    mode: SearchMode,
    cursor: Cursor,
    offset: usize,
    pub count: u16,
}

impl<T> SearchPuzzle<T>
where
    T: Clone + PartialEq<char> + ToString,
    Vec<T>: FromIterator<char>,
{
    pub fn new(content: String) -> Self {
        let rows: Vec<&str> = content.lines().collect();
        let puzzle: Vec<Vec<T>> = rows.iter().map(|row| row.chars().collect()).collect();
        let mode = SearchMode::default();
        let cursor = Cursor { x: 0, y: 0 };
        Self {
            puzzle,
            mode,
            cursor,
            offset: 0,
            count: 0,
        }
    }

    pub fn with_searchmode(mut self, mode_str: &str) -> Self {
        self.mode = SearchMode::from_str(mode_str).unwrap_or_else(|| {
            println!("search mode does not exist. defaulting to 'FULL' search.",);
            SearchMode::default()
        });
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn find_matches(&mut self, word: &str) {
        let puzzle = self.puzzle.clone();
        let characters = word.chars().collect::<Vec<char>>();
        puzzle.iter().enumerate().for_each(|(x, row)| {
            row.iter().enumerate().for_each(|(y, letter)| {
                if *letter == characters[self.offset] {
                    self.cursor.point(x, y);
                    self.check_for_match(word)
                }
            });
        });
    }

    fn check_for_match(&mut self, word: &str) {
        let mut possibilities: Possibilities = Vec::new();
        match self.mode {
            SearchMode::Full => {
                possibilities.extend(self.search_diagonal());
                possibilities.extend(self.search_horizontal_and_vertical());
            }
            SearchMode::Diagonal => possibilities.extend(self.search_diagonal()),
            SearchMode::HorzVert => possibilities.extend(self.search_horizontal_and_vertical()),
        }

        possibilities.iter().for_each(|possible| match possible {
            p if p == word => {
                self.count += 1;
            }
            _ => (),
        });
    }

    fn search_horizontal_and_vertical(&self) -> Possibilities {
        let (x, y) = (self.cursor.x, self.cursor.y);
        let puzzle = &self.puzzle;
        let row = puzzle[x].clone();
        let current_cell = row.get(y).cloned();
        let mut options: Vec<Vec<Option<T>>> = Vec::new();
        let mut possibilities: Possibilities = Vec::new();

        let forward = vec![
            row.get(y).cloned(),
            row.get(y + 1).cloned(),
            row.get(y + 2).cloned(),
            row.get(y + 3).cloned(),
        ];

        options.push(forward);

        let backward = vec![
            current_cell.clone(),
            row.get(y.checked_sub(1).unwrap_or(usize::MAX)).cloned(),
            row.get(y.checked_sub(2).unwrap_or(usize::MAX)).cloned(),
            row.get(y.checked_sub(3).unwrap_or(usize::MAX)).cloned(),
        ];
        options.push(backward);

        let upward = vec![
            current_cell.clone(),
            puzzle
                .get(x.checked_sub(1).unwrap_or(usize::MAX))
                .unwrap_or(&Vec::default())
                .get(y)
                .cloned(),
            puzzle
                .get(x.checked_sub(2).unwrap_or(usize::MAX))
                .unwrap_or(&Vec::default())
                .get(y)
                .cloned(),
            puzzle
                .get(x.checked_sub(3).unwrap_or(usize::MAX))
                .unwrap_or(&Vec::default())
                .get(y)
                .cloned(),
        ];
        options.push(upward);

        let downward = vec![
            current_cell,
            puzzle.get(x + 1).unwrap_or(&Vec::default()).get(y).cloned(),
            puzzle.get(x + 2).unwrap_or(&Vec::default()).get(y).cloned(),
            puzzle.get(x + 3).unwrap_or(&Vec::default()).get(y).cloned(),
        ];
        options.push(downward);

        options.iter_mut().for_each(|opt_vec| {
            let mut possible_match = String::new();
            opt_vec.iter_mut().for_each(|opt| match opt {
                Some(o) => possible_match.push_str(&o.to_string()),
                None => (),
            });
            possibilities.push(possible_match);
        });

        possibilities
    }

    fn search_diagonal(&self) -> Possibilities {
        let (x, y) = (self.cursor.x, self.cursor.y);
        let puzzle = &self.puzzle;
        let row = puzzle[x].clone();
        let current_cell = row.get(y).cloned();
        let mut diag_options: Vec<Vec<Option<T>>> = Vec::new();
        let mut diag_possibilities: Possibilities = Vec::new();

        let upright = vec![
            current_cell.clone(),
            puzzle
                .get(x.checked_sub(1).unwrap_or(usize::MAX))
                .unwrap_or(&Vec::default())
                .get(y + 1)
                .cloned(),
            puzzle
                .get(x.checked_sub(2).unwrap_or(usize::MAX))
                .unwrap_or(&Vec::default())
                .get(y + 2)
                .cloned(),
            puzzle
                .get(x.checked_sub(3).unwrap_or(usize::MAX))
                .unwrap_or(&Vec::default())
                .get(y + 3)
                .cloned(),
        ];
        diag_options.push(upright);

        let downright = vec![
            current_cell.clone(),
            puzzle
                .get(x + 1)
                .unwrap_or(&Vec::default())
                .get(y + 1)
                .cloned(),
            puzzle
                .get(x + 2)
                .unwrap_or(&Vec::default())
                .get(y + 2)
                .cloned(),
            puzzle
                .get(x + 3)
                .unwrap_or(&Vec::default())
                .get(y + 3)
                .cloned(),
        ];
        diag_options.push(downright);

        let upleft = vec![
            current_cell.clone(),
            puzzle
                .get(x.checked_sub(1).unwrap_or(usize::MAX))
                .unwrap_or(&Vec::default())
                .get(y.checked_sub(1).unwrap_or(usize::MAX))
                .cloned(),
            puzzle
                .get(x.checked_sub(2).unwrap_or(usize::MAX))
                .unwrap_or(&Vec::default())
                .get(y.checked_sub(2).unwrap_or(usize::MAX))
                .cloned(),
            puzzle
                .get(x.checked_sub(3).unwrap_or(usize::MAX))
                .unwrap_or(&Vec::default())
                .get(y.checked_sub(3).unwrap_or(usize::MAX))
                .cloned(),
        ];
        diag_options.push(upleft);

        let downleft = vec![
            current_cell.clone(),
            puzzle
                .get(x + 1)
                .unwrap_or(&Vec::default())
                .get(y.checked_sub(1).unwrap_or(usize::MAX))
                .cloned(),
            puzzle
                .get(x + 2)
                .unwrap_or(&Vec::default())
                .get(y.checked_sub(2).unwrap_or(usize::MAX))
                .cloned(),
            puzzle
                .get(x + 3)
                .unwrap_or(&Vec::default())
                .get(y.checked_sub(3).unwrap_or(usize::MAX))
                .cloned(),
        ];
        diag_options.push(downleft);

        diag_options.iter_mut().for_each(|opt_vec| {
            let mut possible_match = String::new();
            opt_vec.iter_mut().for_each(|opt| match opt {
                Some(o) => possible_match.push_str(&o.to_string()),
                None => (),
            });
            diag_possibilities.push(possible_match);
        });

        diag_possibilities
    }
}

#[derive(Clone, Debug)]
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

#[derive(Debug, Default)]
enum SearchMode {
    #[default]
    Full,
    Diagonal,
    HorzVert,
}

impl SearchMode {
    fn from_str(value: &str) -> Option<Self> {
        match value {
            "full" => Some(SearchMode::Full),
            "diag" => Some(SearchMode::Diagonal),
            "hv" => Some(SearchMode::HorzVert),
            _ => None,
        }
    }
}

pub type WordSearch = SearchPuzzle<char>;
type Possibilities = Vec<String>;

