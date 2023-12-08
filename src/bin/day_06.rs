use std::time::Instant;

use aoc2023::read_input;
use nom::{sequence::tuple, character::complete::{u32 as parse_u32, multispace1, newline}, multi::separated_list1, bytes::complete::tag, IResult};


fn parse_input(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    let (i, (_, _, times, _, _, _, distances)) = tuple((
        tag("Time:"),
        multispace1,
        separated_list1(multispace1, parse_u32),
        newline,
        tag("Distance:"),
        multispace1,
        separated_list1(multispace1, parse_u32),
    ))(input)?;
    Ok((i, (times, distances)))
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("06");

    let (_, (times, records)) = parse_input(&input).unwrap();

    let mut product = 1;
    for (time, record) in times.into_iter().zip(records.into_iter()) {
        let mut num_winning = 0;
        for hold_time in 1..time {
            let time_to_race = time - hold_time;
            let distance_traveled = time_to_race * hold_time;
            if distance_traveled > record {
                num_winning += 1;
            }
        }
        product *= num_winning;
    }

    println!("Total time: {:?}", start_time.elapsed());
    println!("{}", product);
}
