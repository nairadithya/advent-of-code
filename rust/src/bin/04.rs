use std::fs;

const INPUT_FILE: &str = "input/02.txt";

fn part_a() {
    let input = fs::read_to_string(INPUT_FILE).expect("Bro give me a real file");
}

fn main() {
    part_a()
}
