use std::fs;

const INPUT_FILE: &str = "input/02.txt";

fn part_a() {
    let input = fs::read_to_string(INPUT_FILE).expect("Bro give me a real file");

    let levels_list = input
        .lines()
        .map(|s| {
            s.split(" ")
                .map(|number| number.parse().expect("That's not a number"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let count = levels_list
        .iter()
        .map(|x| {
            x.windows(2)
                .map(|window| window[1] as i32 - window[0] as i32)
                .all(|diff_item| ((diff_item > 0) && (diff_item <= 3)))
                || x.windows(2)
                    .map(|window| window[1] as i32 - window[0] as i32)
                    .all(|diff_item| (diff_item < 0) && (diff_item >= -3))
        })
        .filter(|&n| n == true)
        .count();

    println!("Solution For Part A: {}", count)
}

fn part_b() {
    let input = fs::read_to_string(INPUT_FILE).expect("Bro give me a real file");

    let levels_list: Vec<Vec<i32>> = input
        .lines()
        .map(|s| {
            s.split(" ")
                .map(|number| number.parse().expect("That's not a number"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let is_safe = |list: &[i32]| {
        let diffs: Vec<i32> = list
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
        diffs.iter().all(|&x| (x < 0) && (x >= -3)) || diffs.iter().all(|&x| (x > 0) && (x <= 3))
    };

    let count = levels_list
        .iter()
        .map(|x| {
            if is_safe(x) {
                true
            } else {
                (0..x.len()).any(|i| {
                    let mut y = x.clone();
                    y.remove(i);
                    is_safe(&y)
                })
            }
        })
        .filter(|&x| x == true)
        .count();

    println!("Solution for PartB: {}", count)
}

fn main() {
    part_a();
    part_b();
}
