use std::fs;

static WINDOW_SIZE: usize = 3;
static INPUT_FILE: &str = "input.txt";


fn part_1(parsed_input: &Vec<i32>) {
    let mut count = 0;
    for index in 1..parsed_input.len() {
        if parsed_input[index] > parsed_input[index - 1] {
            count += 1;
        }
    }
    println!("{}", count);
}

fn calculate_window(array: &Vec<i32>, start_index: usize) -> i32 {
    array[start_index..start_index + WINDOW_SIZE].iter().sum()
}

fn part_2(parsed_input: &Vec<i32>) {
    let mut count = 0;
    for index in 3..parsed_input.len() {
        if calculate_window(parsed_input, index - 2) > calculate_window(parsed_input, index - 3) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    let parsed_input: Vec<i32> = input
        .lines()
        .map(|x| x.parse().expect("Failed to parse input"))
        .collect();
    part_1(&parsed_input);
    part_2(&parsed_input);
}
