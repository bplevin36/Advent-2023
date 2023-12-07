use std::{time::Instant, collections::HashMap};

use aoc2023::read_input;


fn find_adjacent_digits(lines: &[&str], line_idx: usize, col_idx: usize) -> Vec<(usize, usize)> {
    let mut digits = Vec::new();
    if line_idx > 0 {
        let above_start;
        let above_end;
        if col_idx > 0 {
            above_start = col_idx - 1;
        } else {
            above_start = col_idx;
        }
        if col_idx + 1 >= lines[line_idx].len() {
            above_end = col_idx;
        } else {
            above_end = col_idx + 1;
        }
        let above = &lines[line_idx - 1].as_bytes();
        for col in above_start..=above_end {
            let char_to_check = above[col];
            if char_to_check.is_ascii_digit() {
                digits.push((line_idx - 1, col));
            }
        }
    }
    if line_idx + 1 < lines.len() {
        let below_start;
        let below_end;
        if col_idx > 0 {
            below_start = col_idx - 1;
        } else {
            below_start = col_idx;
        }
        if col_idx + 1 >= lines[line_idx].len() {
            below_end = col_idx;
        } else {
            below_end = col_idx + 1;
        }
        let below = &lines[line_idx + 1].as_bytes();
        for col in below_start..=below_end {
            let char_to_check = below[col];
            if char_to_check.is_ascii_digit() {
                digits.push((line_idx + 1, col));
            }
        }
    }
    if col_idx > 0 {
        let char_to_check = lines[line_idx].as_bytes()[col_idx - 1];
        if char_to_check.is_ascii_digit() {
            digits.push((line_idx, col_idx - 1));
        }
    }
    if col_idx + 1 < lines[line_idx].len() {
        let char_to_check = lines[line_idx].as_bytes()[col_idx + 1];
        if char_to_check.is_ascii_digit() {
            digits.push((line_idx, col_idx + 1));
        }
    }
    digits
}

fn num_from_digit_coords(lines: &[&str], line_idx: usize, col_idx: usize) -> (usize, u32) {
    let line = lines[line_idx].as_bytes();
    let mut num_start = col_idx;
    while num_start > 0 && line[num_start - 1].is_ascii_digit() {
        num_start -= 1;
    }
    let mut num_end = col_idx;
    while num_end < line.len() && line[num_end].is_ascii_digit() {
        num_end += 1;
    }
    let num = lines[line_idx][num_start..num_end].parse::<u32>().unwrap();
    (num_start, num)
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("03");
    let start_compute_time = Instant::now();
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    for line_idx in 0..lines.len() {
        let line = lines[line_idx];
        for (col_idx, c) in line.as_bytes().iter().enumerate() {
            if *c == b'*' {
                let adjacents = find_adjacent_digits(&lines, line_idx, col_idx);
                if adjacents.len() > 1 {
                    let mut num_coords_to_nums: HashMap<(usize, usize), u32> = HashMap::new();
                    for (digit_line, digit_col) in adjacents.iter() {
                        let (num_start, num) = num_from_digit_coords(&lines, *digit_line, *digit_col);
                        num_coords_to_nums.insert((*digit_line, num_start), num);
                    }
                    if num_coords_to_nums.len() == 2 {
                        sum += num_coords_to_nums.values().product::<u32>();
                    }
                }

            }
        }
    }
    println!("Total time: {:?}, compute time: {:?}", start_time.elapsed(), start_compute_time.elapsed());
    println!("{}", sum);
}
