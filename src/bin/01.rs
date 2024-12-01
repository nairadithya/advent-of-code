use std::fs;

const INPUT_FILE: &str = "input/01.txt";

fn part1() {
    let input = fs::read_to_string(INPUT_FILE).expect("Got no clue, this ain't no file.");
    let parts: Vec<&str> = input.split_whitespace().collect();

    let mut list1: Vec<i32> = parts
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, element)| element.parse::<i32>().expect("This is not a number my guy"))
        .collect();

    let mut list2: Vec<i32> = parts
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 2 != 1)
        .map(|(_, element)| element.parse::<i32>().expect("This is not a number my guy"))
        .collect();

    list1.sort();
    list2.sort();

    let mut distance_list = Vec::new();
    for (i, _) in list1.iter().enumerate() {
        distance_list.push((list1[i] - list2[i]).abs());
    }

    let sum: i32 = distance_list.iter().sum();

    println!("{}", sum)
}

fn part2() {
    let input = fs::read_to_string(INPUT_FILE).expect("This is definitely not a file.");
}

fn main() {
    part1()
}
