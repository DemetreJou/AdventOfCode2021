use std::fs;
use std::collections::{HashMap, HashSet};

static INPUT_FILE: &str = "input.txt";

fn parse_input(raw_input: &str) -> (&str, Vec<(&str, &str)>) {
    let (start, rules) = raw_input.split_once("\n\n").unwrap();
    let rules: Vec<(&str, &str)> = rules
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            (left, right)
        })
        .collect();
    (start, rules)
}

fn apply_rules(current: &str, rules: &Vec<(&str, &str)>) -> String {
    let mut result = Vec::new();
    let mut matches = HashSet::new();
    for index in 0..current.len() {
        result.push(current.chars().nth(index).unwrap());
        for (pattern, addition) in rules {
            if index + 2 <= current.len() && current[index..index + 2] == **pattern {
                matches.insert(pattern);
                result.push(addition.chars().nth(0).unwrap());
            }
        }
    }
    // pretty_print_set(&matches);
    result.iter().map(|c| *c).collect::<String>()
}

fn part_1(raw_input: &str) {
    let (mut start, rules) = parse_input(raw_input);
    let mut result = String::new();

    for _ in 0..10 {
        let mut map: HashMap<char, usize> = HashMap::new();
        result.chars().for_each(|c| {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        });
        result = apply_rules(start, &rules);
        start = &result;
    }
    let mut map = HashMap::new();
    result.chars().for_each(|c| {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    });
    let most_common = map.iter().max_by_key(|(_, count)| *count).unwrap();
    let least_common = map.iter().min_by_key(|(_, count)| *count).unwrap();
    println!("{}", most_common.1 - least_common.1);
}

fn part_2(raw_input: &str) {
    let (start, rules) = parse_input(raw_input);

    let mut pairs_frequency = HashMap::new();
    for i in 0..start.len() - 1 {
        *pairs_frequency.entry(start[i..i + 2].to_string()).or_insert(0) += 1;
    }

    let mut char_frequency = HashMap::new();
    start.chars().for_each(|c| {
        *char_frequency.entry(c).or_insert(0) += 1 as usize;
    });

    for _ in 0..40 {
        let mut new_map = pairs_frequency.clone();
        for (pattern, addition) in &rules {
            if pairs_frequency.contains_key(&*pattern.to_string())
                && pairs_frequency.get(&*pattern.to_string()).unwrap() >= &1
            {
                let current_pattern_count = pairs_frequency.get(&*pattern.to_string()).unwrap();
                let addition_ch: char = addition.to_string().parse().unwrap();

                *new_map.entry(pattern.parse().unwrap()).or_insert(0) -= current_pattern_count;
                *new_map
                    .entry(pattern.chars().nth(0).unwrap().to_string() + &*addition.to_string())
                    .or_insert(0) += current_pattern_count;
                *new_map
                    .entry(addition.to_string() + &*pattern.chars().nth(1).unwrap().to_string())
                    .or_insert(0) += current_pattern_count;

                *char_frequency.entry(addition_ch).or_insert(1) += current_pattern_count;
            }
        }
        pairs_frequency = new_map;
    }
    let max_key = char_frequency
        .iter()
        .max_by_key(|(_, value)| *value)
        .unwrap();
    let min_key = char_frequency
        .iter()
        .min_by_key(|(_, value)| *value)
        .unwrap();
    println!("{}", max_key.1 - min_key.1);
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
