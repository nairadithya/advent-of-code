use std::fs;

const INPUT_FILE: &str = "input/02.txt";

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("Bro give me a real file");
    let levels_list = input
        .lines()
        .map(|s| {
            s.split(" ")
                .map(|number| number.parse().expect("That's not a number"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let positive_safety_list = levels_list
        .iter()
        .map(|x| {
            x.windows(2)
                .map(|window| window[0] as i32 - window[1] as i32)
                .collect::<Vec<i32>>()
                .iter()
                .all(|&x| x > 0 && x <= 3)
        })
        .collect::<Vec<bool>>();

    let negative_safety_list = levels_list
        .iter()
        .map(|x| {
            x.windows(2)
                .map(|window| window[0] as i32 - window[1] as i32)
                .collect::<Vec<i32>>()
                .iter()
                .all(|&x| x < 0 && x >= -3)
        })
        .collect::<Vec<bool>>();

    let count = positive_safety_list
        .iter()
        .zip(negative_safety_list.iter())
        .map(|(&l1, &l2)| l1 || l2)
        .filter(|&n| n == true)
        .count();
    print!("{}", count);
}
