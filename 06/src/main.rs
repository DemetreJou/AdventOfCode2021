use std::collections::HashMap;
use std::{cmp, fmt, fs};

static INPUT_FILE: &str = "input.txt";

fn perform_one_timestep_naive(mut input: Vec<i32>) -> Vec<i32> {
    let mut num_additional_fish = 0;
    let mut result = input
        .iter()
        .map(|&x| {
            if x == 0 {
                num_additional_fish += 1;
                6
            } else {
                x - 1
            }
        })
        .collect::<Vec<i32>>();
    result.append(&mut vec![8; num_additional_fish]);
    return result;
}

fn part_1(raw_input: &str) {
    let mut input = raw_input
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    (0..80).for_each(|_| {
        input = perform_one_timestep_naive(input.clone());
    });
    println!("{}", input.len());
}

fn perform_one_timestep_smart(m: HashMap<i32, usize>) -> HashMap<i32, usize> {
    let mut result = HashMap::new();
    let mut num_additional_fish = 0;
    for (key, value) in m {
        if key == 0 {
            // num_additional_fish += 1;
            *result.entry(8).or_insert(0) += value;
            *result.entry(6).or_insert(0) += value;
        } else {
            *result.entry(key - 1).or_insert(0) += value;
        }
    }
    // result.insert(8, num_additional_fish);
    return result;
}

fn part_2(raw_input: &str) {
    // can't do obvious approach
    // make buckets for how many fish are each day old
    let mut m: HashMap<i32, usize> = HashMap::new();
    raw_input
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|x| {
            *m.entry(x).or_insert(0) += 1;
        });

    (0..256).for_each(|_| {
        m = perform_one_timestep_smart(m.clone());
    });
    // println!("{:?}", m);
    println!("{}", m.values().sum::<usize>());
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
