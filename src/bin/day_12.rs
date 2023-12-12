use std::{time::Instant, collections::HashMap};

use aoc2023::read_input;
use nom::{IResult, multi::{separated_list1, many1}, character::complete::{newline, multispace1, u32 as parse_u32, one_of}, sequence::separated_pair, bytes::complete::tag, combinator::map};


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
enum Spring {
    Intact,
    Broken,
    Unknown,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Row {
    springs: Vec<Spring>,
    runs: Vec<u8>,
}

fn format_row(row: &Vec<Spring>) -> String {
    let mut ret = String::new();
    for spring in row.iter() {
        ret.push(match spring {
            Spring::Intact => '.',
            Spring::Broken => '#',
            Spring::Unknown => '?',
        });
    }
    ret
}


type Cache<'c> = HashMap<(&'c[Spring], &'c[usize]), usize>;

fn parse_row(input: &str) -> IResult<&str, (Vec<Spring>, Vec<usize>)> {
    let (i, (mut springs, mut runs)) = separated_pair(
        many1(map(one_of(".#?"), |b| match b {
            '.' => Spring::Intact,
            '#' => Spring::Broken,
            '?' => Spring::Unknown,
            _ => panic!("Invalid"),
        })),
        multispace1,
        separated_list1(tag(","), map(parse_u32, |n| n as usize)))(input)?;

    // multiply everything by 5
    let init_springs = springs.len();
    springs.push(Spring::Unknown);
    springs.extend_from_within(..);
    springs.extend_from_within(..);
    springs.extend_from_within(..init_springs);
    runs = runs.repeat(5);

    Ok((i, (springs, runs)))
}


fn attempt_leading_run(mut springs: &[Spring], run_len: usize) -> Option<&[Spring]> {
    for i in 0..run_len {
        if let Some(Spring::Broken | Spring::Unknown) = springs.get(i as usize) {
            springs = &springs[1..];
        }
    }
    if let Some(Spring::Broken) = springs.first() {
        None
    } else {
        Some(springs)
    }
}

fn count_valid<'c>(springs_arg: &'c[Spring], runs: &'c[usize], cache: &mut Cache<'c>) -> usize {
    let mut springs = springs_arg;


    while let [Spring::Intact, rest @ ..] = springs {
        springs = rest;
    }

    // If there are no springs, then there is only an arrangement if there are no blocks.
    if springs.is_empty() {
        return usize::from(runs.is_empty());
    }

    // If there are no blocks, then there is only an arrangement if there are no broken springs.
    if runs.is_empty() {
        return usize::from(springs.iter().all(|s| *s != Spring::Broken));
    }

    let cache_key = (springs, runs);
    if let Some(count) = cache.get(&cache_key) {
        return *count;
    }

    // Easy case: if there are not enough springs to cover the blocks, then there are no arrangements.
    if springs.len() < runs.iter().sum::<usize>() + runs.len() - 1 {
        cache.insert(cache_key, 0);
        return 0;
    }

    if springs[0] == Spring::Unknown {
        // recursively try both possibilities
        let count_if_intact = count_valid(&springs[1..], runs, cache);

        let count_if_broken = match attempt_leading_run(springs, runs[0]) {
            Some(next_springs) => {
                count_valid(&next_springs.get(1..).unwrap_or_default(), &runs[1..], cache)
            },
            None => 0,
        };
        let count = count_if_intact + count_if_broken;
        cache.insert(cache_key, count);
        return count;
    }

    let count = match attempt_leading_run(springs, runs[0]) {
        Some(next_springs) => {
            count_valid(&next_springs.get(1..).unwrap_or_default(), &runs[1..], cache)
        },
        None => 0,
    };
    cache.insert(cache_key, count);
    count
}

fn parse_rows(input: &str) -> IResult<&str, Vec<(Vec<Spring>, Vec<usize>)>> {
    separated_list1(
        newline, parse_row)(input)
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("12_test");

    let (_i, rows) = parse_rows(&input).unwrap();

    let mut cache: Cache = HashMap::new();
    let mut sum = 0;
    for (springs, runs) in rows.iter() {
        let count = count_valid(&springs, &runs, &mut cache);
        println!("Row {} count {}", format_row(springs), count);
        sum += count;
    }
    println!("Total time: {:?}", start_time.elapsed());
    println!("{}", sum);
}
