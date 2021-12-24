use std::collections::HashMap;
use std::fs;

static INPUT_FILE: &str = "test_input.txt";

fn part_1(raw_input: &str) {}

fn part_2(raw_input: &str) {
    let grid = raw_input
        .lines()
        .map(|line| {
            line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
