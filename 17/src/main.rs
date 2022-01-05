use std::{cmp, fs};

static INPUT_FILE: &str = "input.txt";

fn parse_input(input: &str) -> (i32, i32, i32, i32) {
    let (first, second) = input.split_once(", ").unwrap();
    let (_, first) = first.split_once(": ").unwrap();
    let x = first.split_once("=").unwrap().1.split_once("..").unwrap();
    let y = second.split_once("=").unwrap().1.split_once("..").unwrap();
    let x_min = x.0.parse::<i32>().unwrap();
    let x_max = x.1.parse::<i32>().unwrap();
    let y_min = y.0.parse::<i32>().unwrap();
    let y_max = y.1.parse::<i32>().unwrap();
    return (x_min, x_max, y_min, y_max);
}

fn find_min_x_velocity(x_min: i32) -> i32 {
    for i in 0..x_min {
        if (i * (i + 2)) / 2 >= x_min {
            return i;
        }
    }
    return 0;
}

fn apply_single_step(x_velocity: &mut i32, y_velocity: &mut i32, x_pos: &mut i32, y_pos: &mut i32) {
    *x_pos += *x_velocity;
    *y_pos += *y_velocity;
    *x_velocity = if *x_velocity > 0 { *x_velocity - 1 } else { 0 };
    *y_velocity -= 1;
}

fn check_within_target(
    x_pos: i32,
    y_pos: i32,
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
) -> bool {
    return x_pos >= x_min && x_pos <= x_max && y_pos >= y_min && y_pos <= y_max;
}

fn solution(raw_input: &str) {
    let (x_min, x_max, y_min, y_max) = parse_input(raw_input);
    let starting_x = find_min_x_velocity(x_min);
    let mut largest_y = 0;
    let max_y_velocity = cmp::max(y_min.abs(), y_max.abs());
    // not 100% convinced about these bounds, they worked for the test cases though
    let mut num_options = 0;
    for x in starting_x..=x_max {
        for y in -max_y_velocity..max_y_velocity {
            let mut curr_x = 0;
            let mut curr_y = 0;
            let mut x_velocity = x;
            let mut y_velocity = y;
            let mut local_largest_y = curr_y;
            while curr_y >= y_min {
                local_largest_y = cmp::max(local_largest_y, curr_y);
                if check_within_target(curr_x, curr_y, x_min, x_max, y_min, y_max) {
                    largest_y = cmp::max(largest_y, local_largest_y);
                    num_options += 1;
                    break;
                } else {
                    apply_single_step(&mut x_velocity, &mut y_velocity, &mut curr_x, &mut curr_y);
                }
            }
        }
    }
    println!("largest y value for any path: {}", largest_y);
    println!("total number of discint paths: {}", num_options);
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    solution(&input);
}
