use std::time::Instant;

use aoc2023::read_input;
use nom::{sequence::tuple, character::complete::{multispace1, newline, digit1}, multi::separated_list1, bytes::complete::tag, IResult};


fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (i, (_, _, times, _, _, _, distances)) = tuple((
        tag("Time:"),
        multispace1,
        separated_list1(multispace1, digit1),
        newline,
        tag("Distance:"),
        multispace1,
        separated_list1(multispace1, digit1),
    ))(input)?;
    Ok((i, (times, distances)))
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("06");

    let (_, (time_strs, record_strs)) = parse_input(&input).unwrap();

    let time = time_strs.join("").parse::<u64>().unwrap();
    let record = record_strs.join("").parse::<u64>().unwrap();

    let mut num_winning = 0;
    for hold_time in 1..time {
        let time_to_race = time - hold_time;
        let distance_traveled = time_to_race * hold_time;
        if distance_traveled > record {
            num_winning += 1;
        }
    }

    println!("Total time: {:?}", start_time.elapsed());
    println!("{}", num_winning);
}
