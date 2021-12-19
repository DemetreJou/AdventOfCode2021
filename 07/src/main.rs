use std::collections::HashMap;
use std::{cmp, fmt, fs};

static INPUT_FILE: &str = "input.txt";

fn solution(raw_input: &str, is_part_1: bool) {
    let input = raw_input
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut h: HashMap<i32, i32> = HashMap::new();
    input.iter().for_each(|x| *h.entry(*x).or_insert(0) += 1);

    let min = *h.keys().min().unwrap();
    let max = *h.keys().max().unwrap();
    let mut min_fuel_cost = i32::MAX;
    (min..max).for_each(|guess_location| {
        let mut current_fuel_cost = 0;
        for (crab_location, amount) in h.iter() {
            if is_part_1 {
                current_fuel_cost += (crab_location - guess_location).abs() * amount;
            } else {
                // TODO: recalculating this sum is horribly expensive
                // should probably memoize this
                current_fuel_cost +=
                    (0..(guess_location - crab_location).abs() + 1).sum::<i32>() * amount;
            }
        }
        min_fuel_cost = cmp::min(min_fuel_cost, current_fuel_cost);
    });
    println!("{}", min_fuel_cost);
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    solution(&raw_input, true);
    solution(&raw_input, false);
}
