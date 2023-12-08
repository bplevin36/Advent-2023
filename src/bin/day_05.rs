use std::{time::Instant, mem::swap};

use aoc2023::read_input;
use nom::{
    IResult,
    character::complete::{
        u64 as parse_u64, multispace1, not_line_ending, newline
    },
    sequence::{tuple, terminated}, bytes::complete::tag, multi::separated_list1,
};


fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
    let (i, (_, seeds, _)) = tuple((
        tag("seeds: "),
        separated_list1(tag(" "), parse_u64),
        multispace1,
    ))(input)?;
    Ok((i, seeds))
}

fn parse_map_block(input: &str) -> IResult<&str, Vec<(u64, u64, u64)>> {
    let (i, (_, _, mappings, _)) = tuple((
        not_line_ending,
        newline,
        separated_list1(newline,
            tuple((
                terminated(parse_u64, multispace1),
                terminated(parse_u64, multispace1),
                parse_u64))
        ),
        multispace1,
    ))(input)?;
    Ok((i, mappings))
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("05");
    let (mut remaining_input, mut seeds) = parse_seeds(&input).unwrap();
    let mut next_seeds;
    loop {
        next_seeds = seeds.clone();
        match parse_map_block(remaining_input) {
            Err(e) => {
                match e {
                    nom::Err::Error(f) => {
                        if f.input != "" {
                            panic!("Terminated with input remaining: {}", f.input);
                        }
                    },
                    _ => panic!("Error {}", e),
                }
                break;
            },
            Ok((i, mappings)) => {
                remaining_input = i;
                for (dst_start, src_start, length) in mappings {
                    for (&seed, next_seed) in seeds.iter().zip(next_seeds.iter_mut()) {
                        if seed >= src_start && seed < src_start + length {
                            let range_offset = seed - src_start;
                            *next_seed = dst_start + range_offset;
                        }
                    }
                }
            }
        }
        swap(&mut seeds, &mut next_seeds);
    }
    println!("Total time: {:?}", start_time.elapsed());
    println!("{}", seeds.iter().min().unwrap());
}
