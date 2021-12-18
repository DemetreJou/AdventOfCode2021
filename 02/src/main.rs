use std::fs;

static INPUT_FILE: &str = "input.txt";

fn parse_out_number(line: &str) -> i32 {
    line.split_whitespace().collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .expect("Failed to parse input")
}

fn part_1(raw_input: &str) {
    let mut depth = 0;
    let mut horizontal = 0;
    for line in raw_input.lines() {
        match &line[..1] {
            "f" => horizontal += parse_out_number(line),
            "d" => depth += parse_out_number(line),
            "u" => depth -= parse_out_number(line),
            &_ => println!("{}", ""),
        }
    }
    println!("{}", horizontal * depth);
}

fn part_2(raw_input: &str) {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for line in raw_input.lines() {
        match &line[..1] {
            "f" => {
                horizontal += parse_out_number(line);
                depth += aim * parse_out_number(line);
            }
            "d" => aim += parse_out_number(line),
            "u" => aim -= parse_out_number(line),
            &_ => println!("{}", ""),
        }
    }
    println!("{}", horizontal * depth);
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
