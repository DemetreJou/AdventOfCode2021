use std::fs;
use std::collections::HashMap;
use std::f32::RADIX;


static INPUT_FILE: &str = "input.txt";

fn part_1(raw_input: &str) {
    let mut h = HashMap::new();
    h.insert('1', vec![0; raw_input.lines().next().unwrap().len()]);
    h.insert('0', vec![0; raw_input.lines().next().unwrap().len()]);

    for line in raw_input.lines(){
        for (c_index, ch) in line.chars().enumerate(){
            h.get_mut(&ch).unwrap()[c_index] += 1;
        }
    }
    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();

    let line_count = raw_input.lines().count() as i32;
    for count in h[&'1'].iter(){
        if count > &(line_count / 2) {
            gamma.push('1');
        } else {
            gamma.push('0');
        }
    }
    for count in h[&'0'].iter(){
        if count > &(line_count / 2) {
            epsilon.push('1');
        } else {
            epsilon.push('0');
        }
    }

    println!("{:?}", convert_to_decimal(&gamma) * convert_to_decimal(&epsilon));
}

fn convert_to_decimal(input: &Vec<char>) -> u32 {
    input.iter().map(|c| c.to_digit(32).unwrap())
        .fold(0, |acc, x| acc * RADIX + x)
}

fn part_2(raw_input: &str) {
    println!("{}", "Not implemented yet");
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
