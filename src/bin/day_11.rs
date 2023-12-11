use std::time::Instant;

use aoc2023::read_input;

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("11");

    let mut lines: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_owned()).collect();

    // expand empty lines
    for i in (0..lines.len()).rev() {
        let line = &mut lines[i];
        let line_len = line.len();
        if line.iter().all(|&b| b == b'.') {
            lines.insert(i + 1, vec![b'.'; line_len]);
        }
    }

    // expand empty columns
    for col_idx in (0..lines[0].len()).rev() {
        if lines.iter().map(|l| l[col_idx]).all(|b| b == b'.') {
            for line in lines.iter_mut() {
                line.insert(col_idx + 1, b'.')
            }
        }
    }

    // find galaxies
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (line_idx, line) in lines.iter().enumerate() {
        for (col_idx, &byte) in line.iter().enumerate() {
            if byte == b'#' {
                galaxies.push((line_idx, col_idx));
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
