use std::fs;

const INPUT_FILE: &str = "input/04-test.txt";

fn part_a() {
    let input = fs::read_to_string(INPUT_FILE).expect("Bro give me a real file");
    let box_size = input.expect("No newlines here.");

    println!("{}", box_size);
}

fn main() {
    part_a();
}
