use std::collections::HashMap;
use std::fs;

static INPUT_FILE: &str = "input.txt";

fn part_1(raw_input: &str) {
    let h: HashMap<&str, i32> = [(")", 3), ("]", 57), ("}", 1197), (">", 25137)]
        .iter()
        .cloned()
        .collect();

    let opposite: HashMap<&str, &str> = [(")", "("), ("]", "["), ("}", "{"), (">", "<")]
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
        let c  = &c_0.as_str();
        if *c == "(" || *c == "[" || *c == "{" || *c == "<" {
            stack.push(c);
        }
        else {
            let matching_bracket = *opposite.get(c).unwrap();
            match stack.pop(){
                Some(x) => {
                if x == matching_bracket {

                    } else {
                        score += h.get(c).unwrap();
                    }
                }
                None => {}
            }
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
