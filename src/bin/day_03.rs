use std::time::Instant;

use aoc2023::read_input;


fn find_adjacent_symbol(lines: &[&str], line_idx: usize, col_idx: usize) -> Option<(usize, usize, char)> {
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
        for col in above_start..above_end {
            let char_to_check = above[col];
            if char_to_check.is_ascii_punctuation() && char_to_check != b'.' {
                return Some((line_idx - 1, col, char_to_check as char));
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
        for col in below_start..below_end {
            let char_to_check = below[col];
            if char_to_check.is_ascii_punctuation() && char_to_check != b'.' {
                return Some((line_idx + 1, col, char_to_check as char));
            }
        }
    }
    if col_idx > 0 {
        let char_to_check = lines[line_idx].as_bytes()[col_idx - 1];
        if char_to_check.is_ascii_punctuation() && char_to_check != b'.' {
            return Some((line_idx, col_idx - 1, char_to_check as char));
        }
    }
    if col_idx + 1 < lines[line_idx].len() {
        let char_to_check = lines[line_idx].as_bytes()[col_idx + 1];
        if char_to_check.is_ascii_punctuation() && char_to_check != b'.' {
            return Some((line_idx, col_idx + 1, char_to_check as char));
        }
    }
    None
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("03");
    let start_compute_time = Instant::now();
    let mut num_sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    for line_idx in 0..lines.len() {
        let line = lines[line_idx];
        let mut in_num = false;
        let mut found_adjacent_symbol = None;
        let mut num_start = 0;
        for (col_idx, c) in line.as_bytes().iter().enumerate() {
            match (in_num, c.is_ascii_digit()) {
                (true, true) => {
                    let adj = find_adjacent_symbol(&lines, line_idx, col_idx);
                    if adj.is_some() {
                        println!("Found adjacent symbol: {:?}", adj.unwrap());
                        found_adjacent_symbol = adj;
                    }
                },
                (true, false) => {
                    let adj = find_adjacent_symbol(&lines, line_idx, col_idx);
                    if adj.is_some() {
                        println!("Found adjacent symbol: {:?}", adj.unwrap());
                        found_adjacent_symbol = adj;
                    }
                    if found_adjacent_symbol.is_some() {
                        match line[num_start..col_idx].parse::<u32>() {
                            Ok(num_parsed) => {
                                num_sum += num_parsed;
                                println!("Added num {}", num_parsed);
                            },
                            Err(e) => {
                                println!("Failed to parse line {} col {}-{}; '{}': {}", line_idx, num_start, col_idx, &line[num_start..col_idx], e);
                                panic!("panic");
                            }
                        }
                    } else {
                        println!("Not adding {}", &line[num_start..col_idx])
                    }
                    in_num = false;
                    found_adjacent_symbol = None;
                    num_start = 0;
                },
                (false, true) => {
                    in_num = true;
                    num_start = col_idx;
                    println!("Starting in_num iteration: {}", c.escape_ascii().to_string());
                    let adj = find_adjacent_symbol(&lines, line_idx, col_idx);
                    if adj.is_some() {
                        println!("Found adjacent symbol: {:?}", adj.unwrap());
                        found_adjacent_symbol = adj;
                    }
                },
                (false, false) => (),
            }
        }
        if in_num {
            let col_idx = line.len();
            let adj = find_adjacent_symbol(&lines, line_idx, col_idx);
            if adj.is_some() {
                println!("Found adjacent symbol: {:?}", adj.unwrap());
                found_adjacent_symbol = adj;
            }
            if found_adjacent_symbol.is_some() {
                match line[num_start..col_idx].parse::<u32>() {
                    Ok(num_parsed) => {
                        num_sum += num_parsed;
                        println!("Added num {}", num_parsed);
                    },
                    Err(e) => {
                        println!("Failed to parse line {} col {}-{}; '{}': {}", line_idx, num_start, col_idx, &line[num_start..col_idx], e);
                        panic!("panic");
                    }
                }
            } else {
                println!("Not adding {}", &line[num_start..col_idx])
            }
        }
    }
    println!("Total time: {:?}, compute time: {:?}", start_time.elapsed(), start_compute_time.elapsed());
    println!("{}", num_sum);
}
