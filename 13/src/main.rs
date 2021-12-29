use std::fs;

static INPUT_FILE: &str = "input.txt";

fn parse_folds(raw_input: &str) -> Vec<(&str, i32)> {
    raw_input
        .lines()
        .filter(|line| line.contains("fold"))
        .map(|line| {
            let (first, second) = line.split_once("=").unwrap();
            let second = second.parse::<i32>().unwrap();
            match first.contains("x") {
                true => ("x", second),
                false => ("y", second),
            }
        })
        .collect::<Vec<(&str, _)>>()
}

fn parse_points(raw_input: &str) -> Vec<(i32, i32)> {
    raw_input
        .lines()
        .filter(|line| line.contains(","))
        .map(|line| {
            let mut split = line.split(",");
            let x = split.next().unwrap().trim().parse::<i32>().unwrap();
            let y = split.next().unwrap().trim().parse::<i32>().unwrap();
            (x, y)
        })
        .collect::<Vec<_>>()
}

fn apply_single_fold(points: Vec<(i32, i32)>, fold: (&str, i32)) -> Vec<(i32, i32)> {
    let (c, i) = fold;
    points
        .iter()
        .map(|(x, y)| {
            let x = *x;
            let y = *y;
            (x, y)
        })
        .filter_map(|(x, y)| match c {
            "x" if x == i => None,
            "x" if x > i => Some((i - (x - i), y)),
            "y" if y == i => None,
            "y" if y > i => Some((x, i - (y - i))),
            _ => Some((x, y)),
        })
        .collect::<Vec<_>>()
}

fn part_1(raw_input: &str) {
    let points = parse_points(raw_input);
    let folds = parse_folds(raw_input);
    let mut points = apply_single_fold(points, *folds.first().unwrap());
    points.sort_unstable();
    points.dedup();

    println!("{:?}", points.len());
}

fn apply_all_folds(points: Vec<(i32, i32)>, folds: Vec<(&str, i32)>) -> Vec<(i32, i32)> {
    points
        .iter()
        .map(|(x, y)| {
            let x = *x;
            let y = *y;
            (x, y)
        })
        .filter_map(|(mut x, mut y)| {
            for &(c, i) in &folds {
                match c {
                    "x" if x == i => return None,
                    "x" if x > i => x = i - (x - i),
                    "y" if y == i => return None,
                    "y" if y > i => y = i - (y - i),
                    _ => {}
                }
            }
            Some((x, y))
        })
        .collect::<Vec<_>>()
}

fn print_grid(points: Vec<(i32, i32)>) {
    let min_x = points.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = points.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = points.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = points.iter().map(|(_, y)| *y).max().unwrap();

    let mut grid = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];

    for (x, y) in points {
        grid[(y - min_y) as usize][(x - min_x) as usize] = '#';
    }

    for row in grid {
        println!("{:?}", row);
    }
    // PGHRKLKL
}

fn part_2(raw_input: &str) {
    let points = parse_points(raw_input);
    let folds = parse_folds(raw_input);
    let mut points = apply_all_folds(points, folds);
    print_grid(points);
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
