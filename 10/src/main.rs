use std::collections::{HashMap, HashSet};
use std::collections::hash_map::RandomState;
use std::fs;

static INPUT_FILE: &str = "test_input.txt";


fn part_1(raw_input: &str) {
    let h: HashMap<&str, i32> = [
        (")", 3),
        ("]", 57),
        ("}", 1197),
        (">", 25137)
    ]
        .iter()
        .cloned()
        .collect();

    let mut input = Vec::new();
    raw_input.trim()
        .lines()
        .for_each(|l| {
            input.push(
                l.clone().chars().as_str()
            );
        });

    println!("{:?}", input);
    let mut stack: Vec<&str> = Vec::new();
    let mut score = 0;
    input.iter().for_each(|c| {
        if *c == "(" || *c == "[" || *c == "{" || *c == "<" {
            stack.push(c);
        } else if stack.pop().unwrap() == *c {
            // intentionally blank
        } else {
            score += h.get(c).unwrap();
            panic!("Unbalanced brackets");
        }
    });
    println!("{}", score);
}

fn part_2(raw_input: &str) {}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    // part_2(&raw_input);
}
