use std::fs;

const INPUT_FILE: &str = "input/01.txt";

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("Your file doesn't seem to exist my friend.");
    let parts = input.split(" ");
}
