use std::{fmt::Display, time::Instant};

use aoc2023::read_input;
use nom::{IResult, multi::{separated_list1, many1}, character::complete::{newline, multispace1, u8 as parse_u8, one_of}, sequence::separated_pair, bytes::complete::tag, combinator::map};


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Spring {
    Intact,
    Broken,
    Unknown,
}

#[derive(Clone, Debug)]
struct Row {
    springs: Vec<Spring>,
    runs: Vec<u8>,
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for spring in self.springs.iter() {
            write!(f, "{}", match spring {
                Spring::Intact => '.',
                Spring::Broken => '#',
                Spring::Unknown => '?',
            })?;
        }
        Ok(())
    }
}

impl Row {
    fn parse(input: &str) -> IResult<&str, Row> {
        let (i, (springs, runs)) = separated_pair(
            many1(map(one_of(".#?"), |b| match b {
                '.' => Spring::Intact,
                '#' => Spring::Broken,
                '?' => Spring::Unknown,
                _ => panic!("Invalid"),
            })),
            multispace1,
            separated_list1(tag(","), parse_u8))(input)?;
        Ok((i, Row{
            springs,
            runs
        }))
    }

    fn all_possible(&self) -> Vec<Row> {
        let mut possibles = Vec::new();
        possibles.push(self.clone());

        for i in 0..self.springs.len() {
            if self.springs[i] == Spring::Unknown {
                let mut clone_broken = possibles.clone();
                for possible in possibles.iter_mut() {
                    possible.springs[i] = Spring::Intact;
                }
                for possible in clone_broken.iter_mut() {
                    possible.springs[i] = Spring::Broken;
                }
                possibles.append(&mut clone_broken);
            }
        }
        possibles
    }

    fn valid(&self) -> bool {
        let mut run_count = 0;
        let mut run_size = 0;
        for s in self.springs.iter() {
            match s {
                Spring::Intact => {
                    if run_size > 0 {
                        if run_count >= self.runs.len() {
                            return false;
                        }
                        if run_size != self.runs[run_count] {
                            return false;
                        }
                        run_count += 1;
                        run_size = 0;
                    }
                },
                Spring::Broken => {
                    run_size += 1;
                },
                Spring::Unknown => panic!("Invalid"),
            }

        }
        if run_size > 0 {
            if run_count >= self.runs.len() {
                return false;
            }
            if run_size != self.runs[run_count] {
                return false;
            }
            run_count += 1;
        }
        run_count == self.runs.len()
    }
}

fn parse_rows(input: &str) -> IResult<&str, Vec<Row>> {
    separated_list1(
        newline, Row::parse)(input)
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("12");

    let (_i, rows) = parse_rows(&input).unwrap();
    let mut sum = 0;
    for row in rows.iter() {
        let possibles = row.all_possible();
        for possible_row in possibles {
            if possible_row.valid() {
                sum += 1;
            }
        }
    }
    println!("Total time: {:?}", start_time.elapsed());
    println!("{}", sum);
}
