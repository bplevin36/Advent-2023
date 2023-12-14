use std::{time::Instant, collections::HashMap};

use aoc2023::read_input;

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("14");

    let mut lines: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_owned()).collect();

    let mut cycle_loads: Vec<usize> = Vec::new();
    let mut lines_to_cycle: HashMap<Vec<Vec<u8>>, usize> = HashMap::new();

    let mut horiz_limits = vec![0; lines[0].len()];
    let mut vert_limits = vec![0; lines.len()];
    for cycle in 0..1_000_000_000 {
        if let Some(&earlier_cycle) = lines_to_cycle.get(&lines) {
            // returned to earlier state
            // now figure out where in cycle the billionth iteration would end
            let period = cycle - earlier_cycle;
            let cycles_needed = 1_000_000_000 - cycle;
            let ending_index_within_cycle = cycles_needed % period;
            let final_load = cycle_loads[earlier_cycle + ending_index_within_cycle];
            println!("{}", final_load);
            println!("Total time: {:?}", start_time.elapsed());
            return;
        } else {
            let load = get_load(&lines);
            cycle_loads.push(load);
            lines_to_cycle.insert(lines.clone(), cycle_loads.len() - 1);
        }
        spin_cycle(&mut lines, &mut horiz_limits, &mut vert_limits);
    }
}

fn get_load(lines: &Vec<Vec<u8>>) -> usize {
    let mut north_load = 0;
    for (row, line) in lines.iter().enumerate() {
        for byte in line.iter().copied() {
            if byte == b'O' {
                north_load += lines.len() - row;
            }
        }
    }
    north_load
}

fn spin_cycle(lines: &mut Vec<Vec<u8>>, horiz_limits: &mut Vec<usize>, vert_limits: &mut Vec<usize>) {
    horiz_limits.fill(0);
    // roll north
    for line_num in 0..lines.len() {
        for i in 0..lines[line_num].len() {
            let byte = lines[line_num][i];
            match byte {
                b'.' => (),
                b'#' => {
                    horiz_limits[i] = line_num + 1;
                },
                b'O' => {
                    let post_rolling_row = horiz_limits[i];
                    horiz_limits[i] += 1;
                    lines[line_num][i] = b'.';
                    lines[post_rolling_row][i] = b'O';
                }
                _ => panic!("Invalid input")
            }
        }
    }
    // roll west
    vert_limits.fill(0);
    for col in 0..lines[0].len() {
        for row in 0..lines.len() {
            let byte = lines[row][col];
            match byte {
                b'.' => (),
                b'#' => {
                    vert_limits[row] = col + 1;
                },
                b'O' => {
                    let post_rolling_col = vert_limits[row];
                    vert_limits[row] += 1;
                    lines[row][col] = b'.';
                    lines[row][post_rolling_col] = b'O';
                }
                _ => panic!("Invalid input")
            }
        }
    }
    // roll south
    horiz_limits.fill(lines.len() - 1);
    for line_num in (0..lines.len()).rev() {
        for i in 0..lines[line_num].len() {
            let byte = lines[line_num][i];
            match byte {
                b'.' => (),
                b'#' => {
                    horiz_limits[i] = line_num - 1;
                },
                b'O' => {
                    let post_rolling_row = horiz_limits[i];
                    horiz_limits[i] -= 1;
                    lines[line_num][i] = b'.';
                    lines[post_rolling_row][i] = b'O';
                }
                _ => panic!("Invalid input")
            }
        }
    }
    // roll east
    vert_limits.fill(lines[0].len() - 1);
    for col in (0..lines[0].len()).rev() {
        for row in 0..lines.len() {
            let byte = lines[row][col];
            match byte {
                b'.' => (),
                b'#' => {
                    vert_limits[row] = col - 1;
                },
                b'O' => {
                    let post_rolling_col = vert_limits[row];
                    vert_limits[row] -= 1;
                    lines[row][col] = b'.';
                    lines[row][post_rolling_col] = b'O';
                }
                _ => panic!("Invalid input")
            }
        }
    }
}
