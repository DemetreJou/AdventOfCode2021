use std::fs;

use pathfinding::directed::dijkstra;

static INPUT_FILE: &str = "input.txt";

static DIRECTIONS: &[(i32, i32)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

fn dijkstra_wrapper(grid: Vec<Vec<usize>>) -> Option<(Vec<(i32, i32)>, usize)> {
    let goal = ((grid[0].len() - 1), (grid.len() - 1));
    dijkstra::dijkstra(
        &(0, 0),
        |(x, y)| {
            DIRECTIONS
                .iter()
                .map(|(dx, dy)| (x + dx, y + dy))
                .filter(|(x, y)| {
                    *x >= 0 && *y >= 0 && *x < grid[0].len() as i32 && *y < grid.len() as i32
                })
                .map(|(x, y)| ((x, y), grid[y as usize][x as usize]))
                .collect::<Vec<_>>()
        },
        |&p| p.1 as usize == goal.1 && p.0 as usize == goal.0,
    )
}

fn parse_to_grid(raw_input: &str) -> Vec<Vec<usize>> {
    raw_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part_1(raw_input: &str) {
    let grid = parse_to_grid(raw_input);
    match dijkstra_wrapper(grid) {
        Some((_, cost)) => println!("{}", cost),
        None => println!("No path found"),
    }
}

fn part_2(raw_input: &str) {
    let grid = parse_to_grid(raw_input);
    let mut final_grid = grid.clone();

    for i in 0..4 {
        for row_idx in 0..grid.len() {
            final_grid[row_idx].append(
                &mut grid[row_idx]
                    .iter()
                    .map(|&c| c + i + 1)
                    .map(|c| if c >= 10 { c - 9 } else { c })
                    .collect::<Vec<_>>(),
            );
        }
    }

    for row_idx in 0..grid.len() * 5 {
        if row_idx <= grid.len() - 1 {
            continue;
        }
        let row = final_grid[row_idx - grid.len()]
            .clone()
            .iter()
            .map(|&c| if c < 9 { c + 1 } else { 1 })
            .collect::<Vec<_>>();
        final_grid.push(row);
    }

    match dijkstra_wrapper(final_grid) {
        Some((_, cost)) => println!("{}", cost),
        None => println!("No path found"),
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&input);
    part_2(&input);
}
