use std::time::Instant;

use aoc2023::read_input;
use nom::{multi::separated_list1, character::complete::{i32 as parse_i32, newline}, bytes::complete::tag, IResult};


fn parse_sequences(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(newline,
        separated_list1(tag(" "), parse_i32))(input)
}

fn extrapolate_next(sequence: Vec<i32>) -> i32 {
    let mut diffs_vec = Vec::new();
    let mut diffs: Vec<i32> = sequence.windows(2).map(|s| s[1] - s[0]).collect();
    diffs_vec.push(sequence);
    while diffs.iter().any(|&v| v != 0) {
        let next_diffs: Vec<i32> = diffs.windows(2).map(|s| s[1] - s[0]).collect();
        diffs_vec.push(diffs);
        diffs = next_diffs;
    }
    let mut last_delta = 0;
    for diffs in diffs_vec.iter_mut().rev() {
        let next_delta = diffs.last().unwrap() + last_delta;
        diffs.push(next_delta);
        last_delta = next_delta;
    }
    *diffs_vec[0].last().unwrap()
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("09");
    let (_, sequences) = parse_sequences(&input).unwrap();

    let mut sum = 0;
    for sequence in sequences {
        sum += extrapolate_next(sequence);
    }

    println!("Total time: {:?}", start_time.elapsed());
    println!("{}", sum);

}
