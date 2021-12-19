use std::collections::HashMap;
use std::{cmp, fmt, fs};

static INPUT_FILE: &str = "input.txt";

fn part_1(raw_input: &str) {
    let count: usize = raw_input
        .trim()
        .lines()
        // the final .split_whitespace is to turn the string into an iterator
        // would prefer to pipe into the vector constructor but Rust doesn't have that pipe feature
        .flat_map(|line| line.split("|").collect::<Vec<&str>>()[1].split_whitespace())
        .filter(|line| matches!(line.len(), 2 | 3 | 4 | 7))
        .count();
    println!("{:?}", count);
}

fn part_2(raw_input: &str) {}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
