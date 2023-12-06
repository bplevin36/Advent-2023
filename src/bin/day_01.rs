use aoc2023::read_input;
use std::time::Instant;

fn find_digit(line: &[u8], index_iter: impl Iterator<Item=usize>) -> u32 {
    for i in index_iter {
        match line[i] {
            b'0' => return 0,
            b'1' => return 1,
            b'2' => return 2,
            b'3' => return 3,
            b'4' => return 4,
            b'5' => return 5,
            b'6' => return 6,
            b'7' => return 7,
            b'8' => return 8,
            b'9' => return 9,
            _ => (),
        };
        if i + 5 <= line.len() {
            match line[i..i + 5] {
                [b't', b'h', b'r', b'e', b'e'] => return 3,
                [b's', b'e', b'v', b'e', b'n'] => return 7,
                [b'e', b'i', b'g', b'h', b't'] => return 8,
                _ => (),
            }
        }
        if i + 4 <= line.len() {
            match line[i..i + 4] {
                [b'z', b'e', b'r', b'o'] => return 0,
                [b'f', b'o', b'u', b'r'] => return 4,
                [b'f', b'i', b'v', b'e'] => return 5,
                [b'n', b'i', b'n', b'e'] => return 9,
                _ => (),
            }
        }
        if i + 3 <= line.len() {
            match line[i..i + 3] {
                [b'o', b'n', b'e'] => return 1,
                [b't', b'w', b'o'] => return 2,
                [b's', b'i', b'x'] => return 6,
                _ => (),
            };
        }
    }
    0
}

fn num_for_line(line: &[u8]) -> u32 {
    let first_digit = find_digit(line, 0..line.len());
    let last_digit = find_digit(line, (0..line.len()).rev());
    first_digit * 10 + last_digit
}

pub fn run() -> u32 {
    let input = read_input("01");
    let mut sum = 0;
    for line in input.lines() {
        let num = num_for_line(line.as_bytes());
        sum += num;
    }
    sum
}
pub fn main() {
    let runs = 100;
    let start = Instant::now();
    for _ in 0..runs {
        let sum = run();
        if sum == 0 {
            panic!("no");
        }
        println!("{}", sum);
    }

    let duration = start.elapsed();
    println!("{} runs took {:?}", runs, duration);
}
