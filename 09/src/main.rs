use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::fs;

static INPUT_FILE: &str = "input.txt";

fn part_1(raw_input: &str) {
    let lines: Vec<Vec<u32>> = raw_input
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut result = 0;
    let neighbours = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for row_idx in 0..lines.len() {
        for col_idx in 0..lines[row_idx].len() {
            let mut low_point = true;
            for (delta_row, delta_col) in neighbours.iter() {
                let neighbour_row = row_idx as i32 + delta_row;
                let neighbour_col = col_idx as i32 + delta_col;
                if neighbour_row < 0
                    || neighbour_row >= lines.len() as i32
                    || neighbour_col < 0
                    || neighbour_col >= lines[row_idx].len() as i32
                {
                    continue;
                }
                if lines[neighbour_row as usize][neighbour_col as usize] <= lines[row_idx][col_idx]
                {
                    low_point = false;
                }
            }

            if low_point {
                result += lines[row_idx][col_idx] + 1
            }
        }
    }
    println!("{}", result);
}

fn dfs(r: i32, c: i32, grid: &Vec<Vec<u32>>, visited: &mut HashSet<(i32, i32)>) -> i32 {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    if visited.contains(&(r, c)) {
        return 0;
    }
    visited.insert(*&(r, c));
    let mut curr_size = 1;
    for (delta_row, delta_col) in directions.iter() {
        let new_r = r + delta_row;
        let new_c = c + delta_col;
        if new_r < 0
            || new_r >= grid.len() as i32
            || new_c < 0
            || new_c >= grid[0].len() as i32
            || grid[new_r as usize][new_c as usize] == 9
        {
            continue;
        } else {
            curr_size += dfs(new_r, new_c, grid, visited);
        }
    }
    return curr_size;
}

fn part_2(raw_input: &str) {
    let grid: Vec<Vec<u32>> = raw_input
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let mut visited: HashSet<(i32, i32)> = HashSet::with_capacity(grid.len() * grid[0].len());
    let mut all_basins = Vec::new();

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == 9 {
                continue;
            }
            let basin_size = dfs(r as i32, c as i32, &grid, &mut visited);
            all_basins.push(basin_size);
        }
    }
    all_basins.sort();
    all_basins.reverse();
    // println!("{:?}", all_basins);;
    assert!(all_basins.len() >= 3);
    println!("{:?}", all_basins[0..3].iter().product::<i32>());
}

fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
