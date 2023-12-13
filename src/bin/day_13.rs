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
        match (find_horizontal_reflection(pattern), find_vertical_reflection(pattern)) {
            (None, None) | (Some(_), Some(_)) => panic!("Invalid"),
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

fn find_vertical_reflection(pattern: &[&[u8]]) -> Option<usize> {
    for col in 1..pattern[0].len() {
        let mut reflection_refuted = false;
        for line in pattern {
            if !line[col..].iter().zip(line[..col].iter().rev()).all(|(b, a)| b == a) {
                reflection_refuted = true;
                break;
            }
        }

        if !reflection_refuted {
            return Some(col)
        }
    }
    None
}

fn find_horizontal_reflection(pattern: &[&[u8]]) -> Option<usize> {
    for row in 1..pattern.len() {
        let rows_before = &pattern[..row];
        let rows_after = &pattern[row..];
        let mirrored = rows_after.iter()
            .zip(rows_before.iter().rev())
            .all(|(ra, rb)| ra == rb);
        if mirrored {
            return Some(row);
        }
    }
    None
}
