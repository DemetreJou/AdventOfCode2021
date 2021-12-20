use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fs;

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

fn part_2(raw_input: &str) {
    let result = raw_input
        .trim()
        .lines()
        .map(|line| {
            let parts = line.split("|").collect::<Vec<&str>>();
            let input: Vec<&str> = parts[0].split_whitespace().collect();
            let output: Vec<&str> = parts[1].split_whitespace().collect();

            let one = HashSet::<_>::from_iter(
                input
                    .iter()
                    .filter(|n| n.len() == 2)
                    .flat_map(|n| n.chars()),
            );

            let four = HashSet::<_>::from_iter(
                input
                    .iter()
                    .filter(|n| n.len() == 4)
                    .flat_map(|n| n.chars()),
            );

            let parsed_output: Vec<usize> = output
                .iter()
                .map(|n| {
                    let s = HashSet::from_iter(n.chars());
                    match (n.len(), s.is_superset(&one), s.is_superset(&four)) {
                        (2, _, _) => 1,
                        (5, true, false) => 3,
                        (4, _, _) => 4,
                        (6, false, false) => 6,
                        (3, _, _) => 7,
                        (7, _, _) => 8,
                        (6, true, true) => 9,
                        (6, true, false) => 0,
                        _ => {
                            let num_in_common = s.intersection(&four).count();
                            match num_in_common {
                                2 => 2,
                                3 => 5,
                                _ => panic!("unexpected number"),
                            }
                        }
                    }
                })
                .collect();
            let mut sum = 0;
            for (index, val) in parsed_output.iter().enumerate() {
                // this casting is wacky but it works
                sum += *val * 10_usize.pow((parsed_output.len() - index - 1) as u32)
            }
            sum as i32
        })
        .sum::<i32>();

    println!("{:?}", result);
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
