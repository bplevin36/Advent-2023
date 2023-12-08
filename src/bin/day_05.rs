use std::{cmp::Ordering, time::Instant, mem::swap};

use aoc2023::read_input;
use nom::{
    IResult,
    character::complete::{
        u64 as parse_u64, multispace1, not_line_ending, newline
    },
    sequence::{tuple, terminated, separated_pair}, bytes::complete::tag, multi::separated_list1,
};

#[derive(Clone, Default, Debug)]
struct Seeds {
    ranges: Vec<(u64, u64)>,
}

struct Mapping {
    dst_start: u64,
    src_start: u64,
    src_end: u64,
}

impl Seeds {
    pub fn from_input(input: &str) -> IResult<&str, Seeds> {
        let (i, (_, ranges, _)) = tuple((
            tag("seeds: "),
            separated_list1(tag(" "), separated_pair(parse_u64, tag(" "), parse_u64)),
            multispace1,
        ))(input)?;
        Ok((i, Seeds {
            ranges: ranges.into_iter().map(|(s, l)| (s, s + l)).collect()
        }))
    }

    pub fn map_ranges(&self, mappings: &[Mapping]) -> Seeds {
        let mut unmapped_ranges = self.ranges.clone();
        let mut remapped_ranges: Vec<(u64, u64)> = Vec::new();
        for mapping in mappings.iter() {
            let mut i = 0;
            while i < unmapped_ranges.len() {
                let seed_range = unmapped_ranges[i];
                let mut remapped_range;
                match (
                    mapping.src_start.cmp(&seed_range.0),
                    mapping.src_end.cmp(&seed_range.0),
                    mapping.src_start.cmp(&seed_range.1),
                    mapping.src_end.cmp(&seed_range.1),
                ) {
                    (_, Ordering::Less | Ordering::Equal, _, _) |
                    (_, _, Ordering::Greater | Ordering::Equal, _) => {
                        // mapping is strictly smaller than seed range or strictly larger so do nothing
                        i += 1;
                        continue;
                    },
                    (Ordering::Less | Ordering::Equal, Ordering::Greater, _, Ordering::Less) => {
                        // mapping partially left overlaps range
                        remapped_range = (seed_range.0, mapping.src_end);
                        let nonmapped_range = (mapping.src_end, seed_range.1);
                        unmapped_ranges[i] = nonmapped_range;
                        i += 1;
                    },
                    (Ordering::Greater, _, Ordering::Less, Ordering::Greater | Ordering::Equal) => {
                        // mapping partially right overlaps range
                        remapped_range = (mapping.src_start, seed_range.1);
                        let nonmapped_range = (seed_range.0, mapping.src_start);
                        unmapped_ranges[i] = nonmapped_range;
                        i += 1;
                    },
                    (Ordering::Greater, _, Ordering::Less, Ordering::Less) => {
                        // mapping is internal to seed range!
                        remapped_range = (mapping.src_start, mapping.src_end);
                        let nonmapped_range1 = (seed_range.0, mapping.src_start);
                        let nonmapped_range2 = (mapping.src_end, seed_range.1);
                        unmapped_ranges[i] = nonmapped_range1;
                        unmapped_ranges.push(nonmapped_range2);
                        i += 1;
                    },
                    (Ordering::Less | Ordering::Equal, _, _, Ordering::Greater | Ordering::Equal) => {
                        // seed range is entirely remapped
                        remapped_range = (seed_range.0, seed_range.1);
                        // remove from unmapped and don't increment loop
                        unmapped_ranges.remove(i);
                    },
                }
                let mapping_delta = mapping.dst_start as i64 - mapping.src_start as i64;
                remapped_range.0 = remapped_range.0.saturating_add_signed(mapping_delta);
                remapped_range.1 = remapped_range.1.saturating_add_signed(mapping_delta);
                remapped_ranges.push(remapped_range);
            }
        }
        // any ranges that weren't mapped are passed on to the next stage unmodified
        remapped_ranges.append(&mut unmapped_ranges);
        let next_seeds = Seeds { ranges: remapped_ranges };
        next_seeds
    }
}

fn mappings_from_input(input: &str) -> IResult<&str, Vec<Mapping>> {
    let (i, (_, _, tuples, _)) = tuple((
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
    let mappings: Vec<_> = tuples.into_iter()
        .map(|(dst_start, src_start, length)| Mapping { dst_start, src_start, src_end: src_start + length })
        .collect();
    Ok((i, mappings))
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("05");
    let (mut remaining_input, mut seeds) = Seeds::from_input(&input).unwrap();

    let mut next_seeds;
    loop {
        match mappings_from_input(remaining_input) {
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
                next_seeds = seeds.map_ranges(&mappings);
            }
        }
        swap(&mut seeds, &mut next_seeds);
    }
    println!("Total time: {:?}", start_time.elapsed());
    println!("{}", seeds.ranges.iter().map(|t| t.0).min().unwrap());
}
