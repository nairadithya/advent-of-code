use std::fs;

const INPUT_FILE: &str = "input/01.txt";

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("Got no clue, this ain't no file.");

    let mut list1 = input
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, element)| element.parse::<i32>().expect("This is not a number my guy"))
        .collect::<Vec<i32>>()

    let mut list2 = input
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
        .filter(|(index, _)| index % 2 != 1)
        .map(|(_, element)| element.parse::<i32>().expect("This is not a number my guy"))
        .collect::<Vec<i32>>();

    list1.sort();
    list2.sort();

    let answer_a: i32 = list1
        .iter()
        .zip(&list2)
        .map(|(i1, i2)| (i1 - i2) as i32)
        .fold(0, |sum, x| sum + x);

    println!("Solution for Part A: {}", answer_a);
    let similarity_list: Vec<i32> = list1
        .iter()
        .map(|&i1| list2.iter().filter(|&&i2| i2 == i1).count() as i32)
        .collect();

    let answer_b: i32 = list1
        .iter()
        .zip(similarity_list)
        .map(|(i1, i2)| i1 * i2 as i32)
        .fold(0, |sum, x| sum + x);

    println!("Solution For Part B: {}", answer_b)
}
