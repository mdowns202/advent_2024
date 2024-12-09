use super::XMAS_FILE_PATH;
use std::{fmt::Debug, fs, path::Path, str::FromStr, usize};
use strum_macros::EnumString;

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
    word: &'static str,
    mode: SearchMode,
    cursor: Cursor,
    offset: usize,
    multi: u8,
    pub count: u16,
}

impl<T> SearchPuzzle<T>
where
    T: Clone + Debug + PartialEq<char> + ToString,
    Vec<T>: FromIterator<char>,
{
    pub fn new(content: String) -> Self {
        let rows: Vec<&str> = content.lines().collect();
        let puzzle: Vec<Vec<T>> = rows.iter().map(|row| row.chars().collect()).collect();
        let mode = SearchMode::default();
        let cursor = Cursor { x: 0, y: 0 };
        Self {
            puzzle,
            word: "",
            mode,
            cursor,
            offset: 0,
            multi: 1,
            count: 0,
        }
    }

    pub fn with_searchmode(mut self, mode_str: &str) -> Self {
        self.mode = SearchMode::from_str(mode_str).unwrap_or_else(|e| {
            println!(
                "{}: search mode does not exist. defaulting to 'FULL' search.",
                e
            );
            SearchMode::default()
        });
        self
    }
    pub fn with_multiplier(mut self, mul: u8) -> Self {
        self.multi = mul;
        self
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn find_matches(&mut self, word: &'static str) {
        self.word = word;
        let puzzle = self.puzzle.clone();
        let characters: Vec<char> = word.chars().collect();
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
            SearchMode::HoriVert => possibilities.extend(self.search_horizontal_and_vertical()),
        }

        let mut local_count: u8 = 0;

        possibilities.iter().for_each(|possible| match possible {
            p if p == word => {
                local_count += 1;
                if local_count >= self.multi {
                    self.count += 1;
                    local_count = u8::default()
                }
            }
            _ => (),
        });
    }

    fn search_horizontal_and_vertical(&self) -> Possibilities {
        let mut options: Vec<Vec<Option<T>>> = Vec::new();
        let mut possibilities: Possibilities = Vec::new();

        if let Some(fwd) = self.generate_forward() {
            options.push(fwd);
        }
        if let Some(back) = self.generate_backward() {
            options.push(back)
        }
        if let Some(up) = self.generate_upward() {
            options.push(up)
        }
        if let Some(down) = self.generate_downward() {
            options.push(down)
        }

        options.iter_mut().for_each(|opt_vec| {
            let mut possible_match = String::new();
            opt_vec.iter_mut().for_each(|opt| {
                if let Some(value) = opt {
                    possible_match.push_str(&value.to_string());
                }
            });
            possibilities.push(possible_match);
        });
        possibilities
    }

    fn search_diagonal(&self) -> Possibilities {
        let mut dg_options: Vec<Vec<Option<T>>> = Vec::new();
        let mut dg_possibilities: Possibilities = Vec::new();

        if let Some(upright) = self.generate_upright() {
            dg_options.push(upright);
        }
        if let Some(dnright) = self.generate_downright() {
            dg_options.push(dnright);
        }
        if let Some(dnleft) = self.generate_downleft() {
            dg_options.push(dnleft);
        }
        if let Some(upleft) = self.generate_upleft() {
            dg_options.push(upleft);
        }

        dg_options.iter_mut().for_each(|opt_vec| {
            let mut possible_match = String::new();
            opt_vec.iter_mut().for_each(|opt| {
                if let Some(value) = opt {
                    possible_match.push_str(&value.to_string());
                }
            });
            dg_possibilities.push(possible_match);
        });

        dg_possibilities
    }

    fn generate_forward(&self) -> Option<Vec<Option<T>>> {
        let (pos_x, pos_y) = (self.cursor.x, self.cursor.y);

        let forward: Vec<Option<T>> = (0..self.word.chars().count())
            .map(|i| {
                let y = pos_y
                    .checked_sub(self.offset)
                    .unwrap_or(usize::MAX)
                    .checked_add(i)
                    .unwrap_or(usize::MAX);

                self.puzzle
                    .get(pos_x)
                    .unwrap_or(&Vec::default())
                    .get(y)
                    .cloned()
            })
            .collect();

        //if forward.len() == self.word.chars().count() {
        //    println!("FORWARD ==> {:?}", forward);
        //}

        (!forward.is_empty()).then_some(forward)
    }

    fn generate_backward(&self) -> Option<Vec<Option<T>>> {
        let (pos_x, pos_y) = (self.cursor.x, self.cursor.y);
        let row = &self.puzzle[pos_x].clone();

        // IDK but it worked eventually
        let backward: Vec<Option<T>> = (pos_y
            .checked_sub(
                self.word
                    .chars()
                    .count()
                    .checked_sub(self.offset)
                    .unwrap_or(usize::MAX)
                    - 1,
            )
            .unwrap_or(usize::MAX)
            ..self.cursor.y.checked_add(self.offset).unwrap_or(usize::MAX) + 1)
            .rev()
            .map(|i| row.get(i).cloned())
            .collect();

        //if !backward.is_empty() {
        //    println!("BACKWARD <=== {:?}", backward);
        //}

        (!backward.is_empty()).then_some(backward)
    }

    fn generate_upward(&self) -> Option<Vec<Option<T>>> {
        let (pos_x, pos_y) = (self.cursor.x, self.cursor.y);
        let upward: Vec<Option<T>> = (0..self.word.chars().count())
            .map(|i| {
                let x = pos_x.checked_add(self.offset).unwrap_or(usize::MAX);
                let x = x.checked_sub(i).unwrap_or(usize::MAX);
                self.puzzle
                    .get(x)
                    .unwrap_or(&Vec::default())
                    .get(pos_y)
                    .cloned()
            })
            .collect();

        //if !upward.is_empty() {
        //    println!("UPWARD ⇧⇧⇧ {:?}", upward);
        //}

        (!upward.is_empty()).then_some(upward)
    }

    fn generate_downward(&self) -> Option<Vec<Option<T>>> {
        let (pos_x, pos_y) = (self.cursor.x, self.cursor.y);
        let downward: Vec<Option<T>> = (0..self.word.chars().count())
            .map(|i| {
                let x = pos_x.checked_sub(self.offset).unwrap_or(usize::MAX);
                let x = x.checked_add(i).unwrap_or(usize::MAX);
                self.puzzle
                    .get(x)
                    .unwrap_or(&Vec::default())
                    .get(pos_y)
                    .cloned()
            })
            .collect();

        //if !downward.is_empty() {
        //    println!("DOWNWARD ⇩⇩⇩ {:?}", downward);
        //}

        (!downward.is_empty()).then_some(downward)
    }

    fn generate_upright(&self) -> Option<Vec<Option<T>>> {
        let (pos_x, pos_y) = (self.cursor.x, self.cursor.y);
        let offset = self.offset;
        let word_count = self.word.chars().count();

        let upright: Vec<Option<T>> = (0..word_count)
            .map(|i| {
                let x = pos_x
                    .checked_add(offset)
                    .unwrap_or(usize::MAX)
                    .checked_sub(i)
                    .unwrap_or(usize::MAX);

                let y = pos_y
                    .checked_sub(offset)
                    .unwrap_or(usize::MAX)
                    .checked_add(i)
                    .unwrap_or(usize::MAX);

                self.puzzle
                    .get(x)
                    .unwrap_or(&Vec::default())
                    .get(y)
                    .cloned()
            })
            .collect();

        //if !upright.is_empty() {
        //    println!("UPRIGHT =⇧> {:?}", upright);
        //}

        (!upright.is_empty()).then_some(upright)
    }

    fn generate_downright(&self) -> Option<Vec<Option<T>>> {
        let (pos_x, pos_y) = (self.cursor.x, self.cursor.y);

        let downright: Vec<Option<T>> = (0..self.word.chars().count())
            .map(|i| {
                let x = pos_x
                    .checked_sub(self.offset)
                    .unwrap_or(usize::MAX)
                    .checked_add(i)
                    .unwrap_or(usize::MAX);

                let y = pos_y
                    .checked_sub(self.offset)
                    .unwrap_or(usize::MAX)
                    .checked_add(i)
                    .unwrap_or(usize::MAX);

                self.puzzle
                    .get(x)
                    .unwrap_or(&Vec::default())
                    .get(y)
                    .cloned()
            })
            .collect();

        //if !downright.is_empty() {
        //    println!("DOWNRIGHT =⇩> {:?}", downright);
        //}

        (!downright.is_empty()).then_some(downright)
    }

    fn generate_downleft(&self) -> Option<Vec<Option<T>>> {
        let (pos_x, pos_y) = (self.cursor.x, self.cursor.y);

        let downleft: Vec<Option<T>> = (0..self.word.chars().count())
            .map(|i| {
                let x = pos_x
                    .checked_sub(self.offset)
                    .unwrap_or(usize::MAX)
                    .checked_add(i)
                    .unwrap_or(usize::MAX);

                let y = pos_y
                    .checked_add(self.offset)
                    .unwrap_or(usize::MAX)
                    .checked_sub(i)
                    .unwrap_or(usize::MAX);

                self.puzzle
                    .get(x)
                    .unwrap_or(&Vec::default())
                    .get(y)
                    .cloned()
            })
            .collect();

        //if !downleft.is_empty() {
        //    println!("DOWNLEFT <⇩= {:?}", downleft);
        //}

        (!downleft.is_empty()).then_some(downleft)
    }

    fn generate_upleft(&self) -> Option<Vec<Option<T>>> {
        let (pos_x, pos_y) = (self.cursor.x, self.cursor.y);

        let upleft: Vec<Option<T>> = (0..self.word.chars().count())
            .map(|i| {
                let x = pos_x
                    .checked_add(self.offset)
                    .unwrap_or(usize::MAX)
                    .checked_sub(i)
                    .unwrap_or(usize::MAX);

                let y = pos_y
                    .checked_add(self.offset)
                    .unwrap_or(usize::MAX)
                    .checked_sub(i)
                    .unwrap_or(usize::MAX);

                self.puzzle
                    .get(x)
                    .unwrap_or(&Vec::default())
                    .get(y)
                    .cloned()
            })
            .collect();

        //if !upleft.is_empty() {
        //    println!("UPLEFT <⇩= {:?}", upleft);
        //}
        //
        (!upleft.is_empty()).then_some(upleft)
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

#[derive(Debug, Default, EnumString)]
enum SearchMode {
    #[default]
    #[strum(ascii_case_insensitive)]
    Full,
    #[strum(serialize = "diag", serialize = "dg", ascii_case_insensitive)]
    Diagonal,
    #[strum(serialize = "hove", serialize = "hv", ascii_case_insensitive)]
    HoriVert,
}

pub type WordSearch = SearchPuzzle<char>;
type Possibilities = Vec<String>;
