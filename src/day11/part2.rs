use super::STONE_FILE_PATH;
use std::{fs, path::Path};

pub fn print_big_stone_magnitude() {
    let file_path = Path::new(STONE_FILE_PATH);
    let contents = fs::read_to_string(file_path).unwrap();
    let stones: Vec<u64> = contents
        .trim()
        .split(" ")
        .map(|s| s.parse().expect("Unable to parse contents"))
        .collect();

    let result = blink(stones, 75);
    println!("Result: {}", result.last().unwrap());

    //println!(
    //    "D11P2 | Big Stone Magnitude => {}",
    //    contents.lines().nth(1).unwrap()
    //);
}

fn blink(mut stones: Vec<u64>, n: usize) -> Vec<usize> {
    let mut lengths = Vec::new();
    for _ in 0..n {
        let mut new_stones = Vec::new();
        for &stone in &stones {
            if stone == 0 {
                new_stones.push(1);
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let half = stone_str.len() / 2;
                    let (pebble_1, pebble_2) = stone_str.split_at(half);
                    new_stones.push(pebble_1.parse().unwrap_or(0));
                    new_stones.push(pebble_2.parse().unwrap_or(0));
                } else {
                    new_stones.push(stone * 2024);
                }
            }
        }
        stones = new_stones;
        lengths.push(stones.len());
    }
    lengths
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_contents() {
        let file_path = Path::new(STONE_FILE_PATH);
        let contents = fs::read_to_string(file_path).unwrap();
        println!("{}", contents)
    }
}
