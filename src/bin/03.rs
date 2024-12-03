use regex::Regex;
use std::fs;

const INPUT_FILE: &str = "input/03.txt";

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("Got no clue, this ain't no file.");

    let re: Regex = Regex::new(r"(mul\(\d+,\d+\))").unwrap();

    let numbers = re
        .find_iter(&input)
        .map(|m| {
            m.as_str()
                .trim_matches(|x: char| x.is_alphabetic() || x.is_ascii_punctuation())
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<i32>().unwrap())
                .product::<i32>()
        })
        .sum::<i32>();

    println!("{}", numbers)
}
