use std::{time::Instant, ops::Neg};

use aoc2023::read_input;

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("18");

    let instructions = input.lines().map(|line| {
        let mut fields = line.split(" ");
        let direction = fields.next().unwrap();
        let distance = fields.next().unwrap().parse::<isize>().unwrap();
        let color_field = fields.next().unwrap();
        let color = &color_field[1..color_field.len() - 2];
        (direction, distance, color)
    }).collect::<Vec<(&str, isize, &str)>>();

    // find how big our grid needs to be
    let (mut row, mut col) = (0isize, 0isize);
    let (mut max_row, mut min_row, mut max_col, mut min_col) = (0, 0, 0, 0);
    for &(direction, distance, _) in instructions.iter() {
        match direction {
            "U" => {
                row -= distance;
                min_row = min_row.min(row);
            },
            "D" => {
                row += distance;
                max_row = max_row.max(row);
            },
            "R" => {
                col += distance;
                max_col = max_col.max(col);
            },
            "L" => {
                col -= distance;
                min_col = min_col.min(col);
            },
            _ => panic!("Invalid"),
        }
    }
    let num_rows = (max_row - min_row) as usize + 1;
    let num_cols = (max_col - min_col) as usize + 1;


    let mut grid = vec![vec![b'.'; num_cols]; num_rows];
    // lay down the loop on the grid
    let (start_row, start_col) = (min_row.neg() as usize, min_col.neg() as usize);
    let (mut row, mut col) = (start_row, start_col);
    grid[row][col] = b'#';
    for &(direction, distance, _) in instructions.iter() {
        for _ in 0..distance {
            match direction {
                "U" => {
                    row -= 1;
                },
                "D" => {
                    row += 1;
                },
                "R" => {
                    col += 1;
                },
                "L" => {
                    col -= 1;
                },
                _ => panic!("Invalid"),
            }
            grid[row][col] = b'#';
        }
    }
    // find cells that are inside the loop
    let mut fill_grid = grid.clone();
    for row_idx in 0..grid.len() {
        let mut times_crossed = 0;
        let mut from_south = false;
        let mut from_north = false;
        for col_idx in (0..grid[row_idx].len()).rev() {
            match &grid[row_idx][col_idx] {
                b'.' => {
                    if from_north && from_south {
                        from_north = false;
                        from_south = false;
                        times_crossed += 1;
                    } else {
                        from_north = false;
                        from_south = false;
                    }
                    if times_crossed % 2 == 1 {
                        fill_grid[row_idx][col_idx] = b'#';
                    }
                },
                b'#' => {
                    if row_idx > 0 {
                        if grid[row_idx - 1][col_idx] == b'#' {
                            from_north = true;
                        }
                    }
                    if row_idx + 1 < grid.len() {
                        if grid[row_idx + 1][col_idx] == b'#' {
                            from_south = true;
                        }
                    }
                },
                _ => panic!("Invalid"),
            }
        }
    }

    let num_dug: usize = fill_grid.iter().map(|r| r.iter()).flatten().map(|&c| if c == b'#' {1} else {0}).sum();
    println!("{}", num_dug);
    println!("Total time: {:?}", start_time.elapsed());
}
