use std::time::Instant;

use aoc2023::read_input;

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("13");

    let mut patterns: Vec<Vec<&[u8]>> = Vec::new();
    let mut current_pattern = Vec::new();
    for line in input.lines() {
        if line == "" {
            patterns.push(current_pattern);
            current_pattern = Vec::new();
        } else {
            current_pattern.push(line.as_bytes());
        }
    }
    patterns.push(current_pattern);

    let mut sum = 0;
    for pattern in patterns.iter() {
        match (find_horizontal_reflection_smudged(pattern), find_vertical_reflection_smudged(pattern)) {
            (None, None) => panic!("no reflection"),
            (Some(horiz), Some(vert)) => {
                panic!("Both reflections: {}, {}", horiz, vert);
            },
            (Some(horiz), None) => {
                sum += 100 * horiz;
            },
            (None, Some(vert)) => {
                sum += vert;
            },
        }
    }
    println!("{}", sum);
    println!("Total time: {:?}", start_time.elapsed());
}

fn find_vertical_reflection_smudged(pattern: &[&[u8]]) -> Option<usize> {
    for col in 1..pattern[0].len() {
        let mut smudged = false;
        let mut reflection_refuted = false;
        for line in pattern {
            for (byte_before, byte_after) in line[col..].iter().zip(line[..col].iter().rev()) {
                if byte_before != byte_after {
                    if !smudged {
                        smudged = true;
                    } else {
                        reflection_refuted = true;
                        break;
                    }
                }
            }
            if reflection_refuted {
                break;
            }
        }
        if !reflection_refuted && smudged {
            return Some(col)
        }
    }
    None
}

fn find_horizontal_reflection_smudged(pattern: &[&[u8]]) -> Option<usize> {
    for row in 1..pattern.len() {
        let mut smudged = false;
        let mut reflection_refuted = false;
        let rows_before = &pattern[..row];
        let rows_after = &pattern[row..];
        for (&row_after, row_before) in rows_after.iter().zip(rows_before.iter().rev()) {
            for (byte_after, byte_before) in row_after.iter().zip(row_before.iter()) {
                if byte_before != byte_after {
                    if !smudged {
                        smudged = true;
                    } else {
                        reflection_refuted = true;
                        break;
                    }
                }
            }
            if reflection_refuted {
                break;
            }
        }
        if !reflection_refuted && smudged {
            return Some(row);
        }
    }
    None
}
