use std::fs;

static INPUT_FILE: &str = "input.txt";

static DIRECTIONS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn print_grid(grid: &Vec<Vec<u32>>) {
    for row in grid {
        println!("{:?}", row);
    }
    println!();
}


fn apply_flash(grid: &mut Vec<Vec<u32>>, row: usize, col: usize, num_flashes: &mut i32) {
    *num_flashes += 1;
    grid[row][col] = 0;
    for (delta_row, delta_col) in DIRECTIONS.iter() {
        let new_row = row as i32 + delta_row;
        let new_col = col as i32 + delta_col;

        if new_row >= 0
            && new_row < grid.len() as i32
            && new_col >= 0
            && new_col < grid[0].len() as i32
        {
            if grid[new_row as usize][new_col as usize] > 0 {
                let cell = &mut grid[new_row as usize][new_col as usize];
                *cell += 1;
                if *cell > 9 {
                    apply_flash(grid, new_row as usize, new_col as usize, num_flashes);
                }
            }
        }
    }
}

fn part_1(raw_input: &str) {
    let mut grid = raw_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    // not sure how to do this with pure functional paradigm, going to fall back to imperative style
    let mut num_flashes = 0;
    let mut prev_num_flashes;
    let mut i = 0;
    loop {
        prev_num_flashes = num_flashes;
        for row_idx in 0..grid.len() {
            for col_idx in 0..grid[row_idx].len() {
                grid[row_idx][col_idx] += 1;
            }
        }

        let mut temp_grid = Vec::new();
        for row in grid.iter() {
            let mut temp_row = Vec::new();
            for cell in row.iter() {
                temp_row.push(*cell);
            }
            temp_grid.push(temp_row);
        }

        for (row_idx, row) in grid.iter_mut().enumerate() {
            for (col_idx, cell) in row.iter_mut().enumerate() {
                let should_flash = temp_grid[row_idx][col_idx] > 9;
                if should_flash {
                    apply_flash(&mut temp_grid, row_idx, col_idx, &mut num_flashes);
                }
            }
        }
        grid = temp_grid;
        if i == 99 {
            println!("total num flashes is: {}", num_flashes);
        }
        i += 1;
        if prev_num_flashes + 100 == num_flashes {
            println!("first flash sync at iteration: {}", i);
            break;
        }
    }
}


fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
}
