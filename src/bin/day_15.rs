use std::time::Instant;

use aoc2023::read_input;

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("15");

    // run initialization of boxes
    let mut boxes: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];
    for mut step in input.split(',') {
        step = step.trim_end_matches('\n');

        if let Some((label, focal_length_str)) = step.split_once("=") {
            let focal_length = focal_length_str.parse::<u8>().unwrap();
            let box_idx = run_hash(label);
            let box_to_use = &mut boxes[box_idx as usize];
            match box_to_use.iter().position(|(existing_label, _)| *existing_label == label) {
                Some(idx) => {
                    box_to_use.get_mut(idx).unwrap().1 = focal_length;
                },
                None => {
                    box_to_use.push((label, focal_length));
                },
            }
        } else {
            let label = step.strip_suffix("-").unwrap();
            let box_idx = run_hash(label);
            let box_to_use = &mut boxes[box_idx as usize];
            match box_to_use.iter().position(|(existing_label, _)| *existing_label == label) {
                None => (),
                Some(idx) => {
                    box_to_use.remove(idx);
                },
            }
        }
    }
    // compute power
    let mut sum = 0u32;
    for (lens_box, box_num) in boxes.iter().zip(1..) {
        for (&(_, focal_length), lens_num) in lens_box.iter().zip(1..) {
            sum += box_num * lens_num * focal_length as u32;
        }
    }

    println!("{}", sum);
    println!("Total time: {:?}", start_time.elapsed());
}

fn run_hash(input: &str) -> u8 {
    let mut current = 0u8;
    for &byte in input.as_bytes() {
        match current.checked_add(byte) {
            None => {
                current = (((current as u32 + byte as u32) * 17) % 256) as u8;
            }
            Some(sum) => {
                current = sum.wrapping_mul(17);
            },
        }
    }
    current
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(run_hash("HASH"), 52);
        assert_eq!(run_hash("rn=1"), 30);
        assert_eq!(run_hash("qp-"), 14);
        assert_eq!(run_hash("ot=7"), 231);
    }
}
