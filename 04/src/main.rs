use std::fs;

static INPUT_FILE: &str = "input.txt";

#[derive(Debug)]
#[derive(Clone)]
struct Board {
    mask: Vec<Vec<bool>>,
    grid: Vec<Vec<i32>>,
    has_won: bool,
}

impl Board {
    fn apply_move(&mut self, move_: i32) -> Option<i32> {
        if !self.contains_move(move_) {
            return None;
        }
        self.mark_board(move_);
        match self.check_if_over(move_) {
            Some(score) => {
                self.has_won = true;
                Some(score)
            }
            None => None,
        }
    }

    fn contains_move(&self, move_: i32) -> bool {
        for row in self.grid.iter() {
            if row.contains(&move_) {
                return true;
            }
        }
        return false;
    }

    fn mark_board(&mut self, move_: i32) {
        for (row_num, row) in self.grid.iter().enumerate() {
            match row.iter().position(|&x| x == move_) {
                Some(pos) => {
                    self.mask[row_num][pos] = true;
                }
                None => {}
            }
        }
    }

    fn check_if_over(&self, move_: i32) -> Option<i32> {
        // checks if the game is over
        // if it's over also return the score

        let mut is_over = false;
        // check rows
        for row in self.mask.iter() {
            if row.iter().all(|&x| x) {
                is_over = true;
                break;
            }
        }
        // check columns
        // https://stackoverflow.com/questions/54756166/how-do-i-efficiently-iterate-through-a-vecvect-row-by-row
        let rows = self.grid.len();
        let columns = self.grid[0].len();
        let iter = (0..rows).map(|row_idx| self.mask.iter().flatten().skip(row_idx).step_by(columns));

        for mut col in iter {
            if col.all(|&x| x) {
                is_over = true;
                break;
            }
        }


        if is_over {
            return Some(self.get_unmasked_cell_sum() * move_);
        }
        return None;
    }

    fn get_unmasked_cell_sum(&self) -> i32 {
        let mut score: i32 = 0;
        for (row_num, row) in self.mask.iter().enumerate() {
            for (col_num, cell) in row.iter().enumerate() {
                if !*cell {
                    score += self.grid[row_num][col_num];
                }
            }
        }
        return score;
    }

    fn new(grid: Vec<Vec<i32>>) -> Board {
        let mask = vec![vec![false; grid[0].len()]; grid.len()];
        Board {
            mask,
            grid,
            has_won: false,
        }
    }
}

fn parse_moves(lines: &Vec<&str>) -> Vec<i32> {
    lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn parse_boards(lines: &Vec<&str>) -> Vec<Board> {
    let mut current_grid: Vec<Vec<i32>> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    for line in lines[2..].to_vec() {
        if line.len() == 0 {
            continue;
        }

        let temp: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if current_grid.len() == 5 {
            boards.push(Board::new(current_grid));
            current_grid = vec![temp];
        } else { current_grid.push(temp); }
    }
    boards.push(Board::new(current_grid));
    boards
}

fn part_1(raw_input: &str) {
    let lines: Vec<&str> = raw_input.lines().collect();
    let moves = parse_moves(&lines);
    let mut boards = parse_boards(&lines);
    let mut boards_2 = Vec::new();

    for move_ in moves {
        for mut board in boards {
            match board.apply_move(move_) {
                Some(score) => {
                    println!("{} {}", move_, score);
                    for line in board.mask {
                        println!("{:?}", line);
                    }
                    return;
                }
                None => {}
            }
            boards_2.push(board);
        }
        boards = boards_2;
        boards_2 = Vec::new();
    }
}

fn part_2(raw_input: &str) {
    let lines: Vec<&str> = raw_input.lines().collect();
    let moves = parse_moves(&lines);
    let mut boards: Vec<Board> = parse_boards(&lines);
    let mut boards_2: Vec<Board> = Vec::new();

    let mut winners: Vec<bool> = vec![false; boards.len()];

    for move_ in moves {
        let mut index = 0;
        for mut board in boards {
            match board.apply_move(move_) {
                Some(score) => {
                    winners[index] = true;
                    if winners.iter().all(|&x| x) {
                        println!("{} {}", move_, score);
                        return;
                    }
                }
                None => {}
            }
            boards_2.push(board);
            index += 1;
        }
        index = 0;
        boards = boards_2;
        boards_2 = Vec::new();
    }
}


fn main() {
    let raw_input = fs::read_to_string(INPUT_FILE).expect("Failed to read input.txt");
    part_1(&raw_input);
    part_2(&raw_input);
}
