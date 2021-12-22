use std::collections::HashMap;
use std::fs;

static INPUT_FILE: &str = "input.txt";

fn part_1(raw_input: &str) {
    let score_hashmap: HashMap<&str, i32> = [(")", 3), ("]", 57), ("}", 1197), (">", 25137)]
        .iter()
        .cloned()
        .collect();

    let opposite: HashMap<&str, &str> = [
        (")", "("),
        ("]", "["),
        ("}", "{"),
        (">", "<"),
        ("(", ")"),
        ("[", "]"),
        ("{", "}"),
        ("<", ">"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut input = Vec::new();
    for line in raw_input.lines() {
        for ch in line.chars() {
            input.push(ch.to_string());
        }
    }

    let mut stack: Vec<&str> = Vec::new();
    let mut score = 0;
    input.iter().for_each(|c_0| {
        let c = &c_0.as_str();
        if *c == "(" || *c == "[" || *c == "{" || *c == "<" {
            stack.push(c);
        } else {
            let matching_bracket = *opposite.get(c).unwrap();
            match stack.pop() {
                Some(x) => {
                    if x == matching_bracket {
                    } else {
                        score += score_hashmap.get(c).unwrap();
                    }
                }
                None => {}
            }
        }
    });
    println!("{}", score);
}

fn calculate_score(stack: Vec<&str>, score_hashmap: &HashMap<&str, u128>) -> u128 {
    assert!(stack.len() > 0);
    let mut score: u128 = 0;
    stack.iter().for_each(|c| {
        score = (score * 5) + *score_hashmap.get(c).unwrap() as u128;
    });
    score
}

fn part_2(raw_input: &str) {
    let score_hashmap: HashMap<&str, u128> = [(")", 1), ("]", 2), ("}", 3), (">", 4)]
        .iter()
        .cloned()
        .collect();

    let opposite: HashMap<&str, &str> = [
        (")", "("),
        ("]", "["),
        ("}", "{"),
        (">", "<"),
        ("(", ")"),
        ("[", "]"),
        ("{", "}"),
        ("<", ">"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut input = Vec::new();
    for line in raw_input.lines() {
        let mut tmp = Vec::new();
        for ch in line.chars() {
            tmp.push(ch.to_string());
        }
        input.push(tmp);
    }

    let mut all_scores: Vec<u128> = Vec::new();

    input.iter().for_each(|line| {
        let mut stack: Vec<&str> = Vec::new();
        let mut valid = true;
        line.iter().for_each(|c_0| {
            let c = c_0.as_str();
            if c == "(" || c == "[" || c == "{" || c == "<" {
                stack.push(c);
            } else {
                let matching_bracket = *opposite.get(c).unwrap();
                match stack.pop() {
                    Some(last_element) => {
                        if last_element != matching_bracket {
                            valid = false;
                        }
                    }
                    None => {
                        valid = false;
                    }
                }
            }
        });
        if valid {
            let mut suffix: Vec<&str> = Vec::new();
            for bracket in stack.iter().rev() {
                suffix.push(*opposite.get(bracket).unwrap());
            }
            all_scores.push(calculate_score(suffix, &score_hashmap));
        }
    });
    // not 1891599192, too low
    all_scores.sort();
    let mid = all_scores.len() / 2;
    println!("{:?}", all_scores[mid]);
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
