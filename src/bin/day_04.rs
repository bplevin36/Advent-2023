use std::{collections::HashSet, time::Instant};

use aoc2023::read_input;
use nom::{
    IResult,
    character::complete::{u32 as parse_u32, multispace1, multispace0}, sequence::tuple, bytes::complete::tag, multi::{separated_list1, many1},
};


struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    pub fn from_line(line: &str) -> IResult<&str, Card> {
        let (i, (_, _, id, _, _, winning_numbers, _,_, _, my_numbers)) = tuple((
            tag("Card"),
            multispace1,
            parse_u32,
            tag(":"),
            multispace0,
            separated_list1(multispace1, parse_u32),
            multispace0,
            tag("|"),
            multispace0,
            separated_list1(multispace1, parse_u32),
        ))(line)?;
        Ok((i, Card {
            winning_numbers: winning_numbers.into_iter().collect(),
            my_numbers: my_numbers.into_iter().collect(),
        }))
    }

    pub fn point_value(&self) -> u32 {
        let num_winning = self.winning_numbers.intersection(&self.my_numbers).count() as u32;
        if num_winning == 0 {
            0
        } else {
            2u32.pow(num_winning - 1)
        }
    }
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("04");

    let mut sum = 0;
    for line in input.lines() {
        let (_, card) = Card::from_line(line).unwrap();
        sum += card.point_value();
    }
    println!("Total time: {:?}", start_time.elapsed());
    println!("{}", sum);
}
