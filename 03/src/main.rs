use std::fs;
use std::collections::HashMap;
use std::f32::RADIX;


static INPUT_FILE: &str = "input.txt";

fn convert_to_decimal(input: &str) -> u32 {
    input.chars().map(|c| c.to_digit(32).unwrap())
        .fold(0, |acc, x| acc * RADIX + x)
}

fn num_bits_per_position(lines: &Vec<&str>) -> HashMap<char, Vec<i32>> {
    let mut h = HashMap::new();
    h.insert('1', vec![0; lines[0].len()]);
    h.insert('0', vec![0; lines[0].len()]);
    for line in lines.iter() {
        for (c_index, ch) in line.chars().enumerate() {
            h.get_mut(&ch).unwrap()[c_index] += 1;
        }
    }
    h
}

fn part_1(raw_input: &str) {
    let lines = raw_input.lines().collect::<Vec<&str>>();
    let h = num_bits_per_position(&lines);

    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();

    let num_lines = raw_input.lines().count() as i32;
    for count in h[&'1'].iter() {
        if count > &(num_lines / 2) {
            gamma.push('1');
        } else {
            gamma.push('0');
        }
    }
    for count in h[&'0'].iter() {
        if count > &(num_lines / 2) {
            epsilon.push('1');
        } else {
            epsilon.push('0');
        }
    }
    let gamma_decimal = convert_to_decimal(&gamma.iter().collect::<String>());
    let epsilon_decimal = convert_to_decimal(&epsilon.iter().collect::<String>());
    println!("{:?}", gamma_decimal * epsilon_decimal);
}


fn part_2_helper(lines: Vec<&str>, x: char) -> &str {
    let mut possible_lines = lines.clone();
    let mut most_common: char;

    for i in 0..lines[0].len() as i32 {
        let h = num_bits_per_position(&possible_lines);
        let num_lines: i32 = possible_lines.len() as i32;
        let count: i32 = h[&x][i as usize];
        if possible_lines.len() == 1 {
            break;
        }
        if count * 2 == num_lines {
            most_common = x;
        } else if count * 2 > num_lines {
            most_common = '1';
        } else {
            most_common = '0';
        }
        // println!("{:?} {}", count, num_lines);
        // println!("{:?}\n", most_common);
        possible_lines = possible_lines.into_iter().filter(|line| line.chars().nth(i as usize).unwrap() == most_common).collect();
    }
    return possible_lines[0];
}


fn part_2(raw_input: &str) {
    let lines = raw_input.lines().collect::<Vec<&str>>();
    // Doing this clone because otherwise I get an error with lifetimes that I don't understand...yet
    let gamma = part_2_helper(lines.clone(), '1');
    let epsilon = part_2_helper(lines.clone(), '0');
    println!("{:?}", convert_to_decimal(gamma) * convert_to_decimal(epsilon));
}


fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
