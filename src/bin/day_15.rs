use std::time::Instant;

use aoc2023::read_input;

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("15");

    let mut sum = 0u32;
    for mut step in input.split(',') {
        step = step.trim_end_matches('\n');
        sum += run_hash(step) as u32;
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
