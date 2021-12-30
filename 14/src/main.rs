use std::collections::{HashMap, HashSet};
use std::fs;

static INPUT_FILE: &str = "test_input.txt";

fn parse_input(raw_input: &str) -> (&str, Vec<(&str, &str)>) {
    let (start, rules) = raw_input.split_once("\n\n").unwrap();
    let rules: Vec<(&str, &str)> = rules.lines().map(|line| {
        let (left, right) = line.split_once(" -> ").unwrap();
        (left, right)
    }).collect();
    (start, rules)
}

fn apply_rules(current: &str, rules: &Vec<(&str, &str)>) -> String {
    let mut result = Vec::new();
    for index in 0..current.len() {
        result.push(current.chars().nth(index).unwrap());
        for (pattern, addition) in rules {
            if index + 2 <= current.len() && current[index..index + 2] == **pattern {
                result.push(addition.chars().nth(0).unwrap());
            }
        }
    }
    result.iter().map(|c| *c).collect::<String>()
}

fn part_1(raw_input: &str) {
    let (mut start, rules) = parse_input(raw_input);
    let mut result = String::new();
    for _ in 0..10 {
        result = apply_rules(start, &rules);
        start = &result;
    }
    println!("{}", result.len());
    let mut map = HashMap::new();
    result.chars().for_each(|c| {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    });
    let most_common = map.iter().max_by_key(|(_, count)| *count).unwrap();
    let least_common = map.iter().min_by_key(|(_, count)| *count).unwrap();
    println!("{}", most_common.1 - least_common.1);
}

fn create_frequency_map(map: &HashMap<String, usize>) -> HashMap<String, usize> {
    let mut frequency_count = HashMap::new();
    for (key, value) in map.iter() {
        *frequency_count.entry(key.chars().nth(0).unwrap()).or_insert(0) += f32::ceil(*value as f32/2 as f32) as usize;
        *frequency_count.entry(key.chars().nth(1).unwrap()).or_insert(0) += f32::ceil(*value as f32/2 as f32) as usize;
    }

    // let mut final_frequency = HashMap::new();
    // for (key, value) in frequency_count.iter_mut() {
    //     *final_frequency.entry(key.to_string().parse::<String>().unwrap()).or_insert(0) = f32::ceil(*value as f32/2 as f32) as usize;
    // }
    let final_frequency = frequency_count.iter().map(|(key, value)| (key.to_string().parse::<String>().unwrap(), *value)).collect::<HashMap<String, usize>>();
    final_frequency
}

fn part_2(raw_input: &str) {
    let (mut start, rules) = parse_input(raw_input);

    let mut map = HashMap::new();
    for i in 0..start.len() - 1 {
        map.entry(start[i..i + 2].to_string()).or_insert(1 as usize);
    }
    let mut new_map = map.clone();
    // println!("{:?}", map);

    let frequency_count = create_frequency_map(&map);
    println!("{:?}", frequency_count);

    for _ in 0..40    {
        for (pattern, addition) in &rules {
            if map.contains_key(&*pattern.to_string()) && map.get(&*pattern.to_string()).unwrap() >= &1 {
                let count = map.get(&*pattern.to_string()).unwrap();
                *new_map.entry(pattern.parse().unwrap()).or_insert(0) -= count;
                *new_map.entry(pattern.chars().nth(0).unwrap().to_string() + &*addition.to_string()).or_insert(0) += count;
                *new_map.entry(addition.to_string() + &*pattern.chars().nth(1).unwrap().to_string()).or_insert(0) += count;
            }
        }
        map = new_map.clone();
    }
    let count = map.iter().map(|(_, value)| *value).sum::<usize>();
    println!("{}", count);

    // TODO: notice that the first and last character are always the same
    // can maybe take advantage of the fact that the additions happen between 2 characters

    // maybe instead of calculate the count at the end, keep track of the count of each character as the patterns are being matched
    // when a pattern is matched, pattern[0] and pattern[1] and additional are all increased by the same amount
    let final_frequency = create_frequency_map(&map);
    println!("{:?}", final_frequency);
    let max_key = final_frequency.iter().max_by_key(|(_, value)| *value).unwrap();
    let min_key = final_frequency.iter().min_by_key(|(_, value)| *value).unwrap();
    println!("{}", max_key.1 - min_key.1);
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    // part_1(&raw_input);
    part_2(&raw_input);
}
