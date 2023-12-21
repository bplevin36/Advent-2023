use std::{mem, time::Instant};

use aoc2023::read_input;
use nom::{IResult, multi::{separated_list1, many1}, character::complete::{newline, one_of}, combinator::map};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Plot {
    Garden,
    Rock,
    Reached,
}

impl Plot {
    fn from_char(c: char) -> Plot {
        match c {
            '.' => Plot::Garden,
            '#' => Plot::Rock,
            'S' => Plot::Reached,
            _ => panic!("Invalid"),
        }
    }

    fn set_reached_if_not_rock(&mut self) {
        match self {
            Plot::Rock => (),
            _ => {
                *self = Plot::Reached;
            },
        }
    }
}

fn parse_map(input: &str) -> IResult<&str, Vec<Vec<Plot>>> {
    separated_list1(newline, many1(map(one_of(".#S"), |c| Plot::from_char(c))))(input)
}

fn reset_reachable(map: &mut Vec<Vec<Plot>>) {
    for row in map.iter_mut() {
        for plot in row.iter_mut() {
            if *plot == Plot::Reached {
                *plot = Plot::Garden;
            }
        }
    }
}

const FULL_STEP_COUNT: usize = 64;
fn main() {
    let start_time = Instant::now();
    let input = read_input("21");

    let (_, mut curr_map) = parse_map(&input).unwrap();
    let mut next_map = curr_map.clone();

    for _ in 0..FULL_STEP_COUNT {
        reset_reachable(&mut next_map);
        // find next reachable positions
        for row_idx in 0..curr_map.len() {
            let row = &curr_map[row_idx];
            for col_idx in 0..row.len() {
                let plot = &row[col_idx];
                if let Plot::Reached = plot {
                    if col_idx > 0 {
                        next_map[row_idx][col_idx - 1].set_reached_if_not_rock();
                    }
                    if col_idx + 1 < next_map[row_idx].len() {
                        next_map[row_idx][col_idx + 1].set_reached_if_not_rock();
                    }
                    if row_idx > 0 {
                        next_map[row_idx - 1][col_idx].set_reached_if_not_rock();
                    }
                    if row_idx + 1 < next_map.len() {
                        next_map[row_idx + 1][col_idx].set_reached_if_not_rock();
                    }
                }
            }
        }

        mem::swap(&mut next_map, &mut curr_map);
    }

    let num_reached = curr_map.iter().map(|v| v.iter()).flatten().filter(|&&p| p == Plot::Reached).count();
    println!("{}", num_reached);
    println!("Total time: {:?}", start_time.elapsed());
}
