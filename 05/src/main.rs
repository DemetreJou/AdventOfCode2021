use std::{cmp, fmt, fs};

static INPUT_FILE: &str = "input.txt";

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(points: Vec<Point>) -> Line {
        let mut p = points.clone();
        Line {
            start: p.remove(0),
            end: p.remove(0),
        }
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?}", self.start, self.end)
    }
}

fn parse_lines(input_lines: Vec<&str>) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    for line in input_lines {
        let mut l: Vec<Point> = Vec::new();
        for point in line.split(" -> ").collect::<Vec<&str>>() {
            let p = point.split(",").collect::<Vec<&str>>();
            let x = p[0].parse::<i32>().unwrap();
            let y = p[1].parse::<i32>().unwrap();
            l.push(Point::new(x, y));
        }
        lines.push(Line::new(l));
    }
    lines
}

fn build_matrix(lines: Vec<Line>) -> Vec<Vec<i32>> {
    let max_row = lines.iter().map(|l| l.start.y).max().unwrap() + 1;
    let max_col = lines.iter().map(|l| l.start.x).max().unwrap() + 1;
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; max_col as usize]; max_row as usize];

    for line in &lines {
        let start = &line.start;
        let end = &line.end;
        if start.x == end.x {
            let start_y = cmp::min(start.y, end.y);
            let end_y = cmp::max(start.y, end.y);
            for i in start_y..end_y + 1 {
                matrix[i as usize][start.x as usize] += 1;
            }
        } else if start.y == end.y {
            let start_x = cmp::min(start.x, end.x);
            let end_x = cmp::max(start.x, end.x);
            for i in start_x..end_x + 1 {
                matrix[start.y as usize][i as usize] += 1;
            }
        }
        // extended in part 2 for diagonal lines as well
        else {
            let x_delta = if start.x < end.x { 1 } else { -1 };
            let y_delta = if start.y < end.y { 1 } else { -1 };
            let mut curr_col = start.x;
            let mut curr_row = start.y;
            let line_length = (end.x - start.x).abs();
            for _ in 0..line_length + 1 {
                matrix[curr_row as usize][curr_col as usize] += 1;
                curr_col += x_delta;
                curr_row += y_delta;
            }
        }
    }

    matrix
}

fn count_dangerous_areas(matrix: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for row in matrix {
        for cell in row {
            if cell >= 2 {
                count += 1;
            }
        }
    }
    count
}

fn part_1(raw_input: &str) {
    let input_lines: Vec<&str> = raw_input.lines().collect();
    let lines: Vec<Line> = parse_lines(input_lines)
        .into_iter()
        .filter(|l| l.start.x == l.end.x || l.start.y == l.end.y)
        .collect();

    let matrix = build_matrix(lines);
    let count = count_dangerous_areas(matrix);
    println!("{}", count);
}

fn part_2(raw_input: &str) {
    let input_lines: Vec<&str> = raw_input.lines().collect();
    let lines: Vec<Line> = parse_lines(input_lines);
    let matrix = build_matrix(lines);

    for row in &matrix {
        println!("{:?}", row);
    }

    let count = count_dangerous_areas(matrix);
    println!("{}", count);
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
