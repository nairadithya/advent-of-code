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

    //7 6 4 2 1
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

    let mut levels_list: Vec<Vec<i32>> = input
        .lines()
        .map(|s| {
            s.split(" ")
                .map(|number| number.parse().expect("That's not a number"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let new_levels_list = levels_list.iter().map(|x| {
        x.iter().enumerate().map(|(i, _)| {
            let mut y = x.to_vec();
            y.remove(i);
            y.windows(2)
                .map(|window| window[0] as i32 - window[1] as i32)
                .collect::<Vec<i32>>()
                .iter()
                .all(|&x| ((x < 0) && (x <= 3)))
                || y.windows(2)
                    .map(|window| window[0] as i32 - window[1] as i32)
                    .collect::<Vec<i32>>()
                    .iter()
                    .all(|&x| (x > 0) && (x <= 3))
        })
    });
}

fn main() {
    part_a();
}
