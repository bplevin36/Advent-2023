use std::time::Instant;

use aoc2023::read_input;

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("14");

    let lines: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();

    let mut northern_limits: Vec<usize> = vec![0; lines[0].len()];
    let mut load_sum = 0;
    for (line_num, line) in lines.iter().enumerate() {
        for (i, byte) in line.iter().enumerate() {
            match byte {
                b'.' => (),
                b'#' => {
                    northern_limits[i] = line_num + 1;
                },
                b'O' => {
                    let post_rolling_row = northern_limits[i];
                    northern_limits[i] += 1;
                    let load = lines.len() - post_rolling_row;
                    load_sum += load;
                }
                _ => panic!("Invalid input")
            }
        }
    }

    println!("{}", load_sum);
    println!("Total time: {:?}", start_time.elapsed());
}
