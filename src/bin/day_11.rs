use std::time::Instant;

use aoc2023::read_input;

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("11");

    const EXPANSION_FACTOR: usize = 999_999;
    let mut lines: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_owned()).collect();
    let mut empty_lines: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    // find empty lines
    for i in 0..lines.len() {
        let line = &mut lines[i];
        if line.iter().all(|&b| b == b'.') {
            empty_lines.push(i);
        }
    }

    // find empty columns
    for col_idx in 0..lines[0].len() {
        if lines.iter().map(|l| l[col_idx]).all(|b| b == b'.') {
            empty_cols.push(col_idx);
        }
    }

    // find galaxies
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (line_idx, line) in lines.iter().enumerate() {
        for (col_idx, &byte) in line.iter().enumerate() {
            if byte == b'#' {
                let mut galaxy_line = line_idx;
                let mut galaxy_col = col_idx;
                galaxy_line += match empty_lines.binary_search(&line_idx) {
                    Ok(_num_empty) => panic!("This was supposed to be empty"),
                    Err(num_empty) => num_empty * EXPANSION_FACTOR,
                };
                galaxy_col += match empty_cols.binary_search(&col_idx) {
                    Ok(_num_empty) => panic!("This was supposed to be empty"),
                    Err(num_empty) => num_empty * EXPANSION_FACTOR,
                };
                galaxies.push((galaxy_line, galaxy_col));
            }
        }
    }
    // compute distances
    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        let galaxy = galaxies[i];
        let remaining = &galaxies[i + 1..];
        for other_galaxy in remaining {
            let distance = galaxy.0.abs_diff(other_galaxy.0) + galaxy.1.abs_diff(other_galaxy.1);
            sum += distance;
        }
    }

    println!("Total time: {:?}", start_time.elapsed());
    println!("{}", sum);
}
