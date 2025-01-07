use regex::Regex;
use std::fs;

const INPUT_FILE: &str = "input/03.txt";

fn part_a() {
    let input = fs::read_to_string(INPUT_FILE).expect("Got no clue, this ain't no file.");

    let re: Regex = Regex::new(r"(mul\(\d+,\d+\))").expect("Huh, no matches here.");

    let numbers = re
        .find_iter(&input)
        .map(|m| {
            m.as_str()
                .trim_matches(|x: char| x.is_alphabetic() || x.is_ascii_punctuation())
                .split(",")
                .map(|x| x.parse::<i32>().expect("That's not a number"))
                .product::<i32>()
        })
        .sum::<i32>();

    println!("Solution For Part A: {}", numbers)
}

fn part_b() {
    let input = fs::read_to_string(INPUT_FILE).expect("Got no clue, this ain't no file.");

    let re: Regex = Regex::new(r"do\(\)|mul\(\d+,\d+\)|don't\(\)").expect("Huh, no matches here");

    let list: Vec<&str> = re
        .find_iter(&input)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>();

    let mut flag = true;
    let mut sum = 0;
    for item in list {
        if item == "do()" {
            flag = true;
            continue;
        } else if item == "don't()" {
            flag = false;
            continue;
        } else if flag == true {
            sum += item
                .trim_matches(|x: char| x.is_alphabetic() || x.is_ascii_punctuation())
                .split(",")
                .map(|y| y.parse::<i32>().expect("That's not a number"))
                .product::<i32>();
        }
    }

    println!("Solution For Part B: {}", sum);
}

fn main() {
    part_a();
    part_b();
}
