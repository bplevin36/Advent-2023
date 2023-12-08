use std::{collections::{HashSet, HashMap}, time::Instant};

use aoc2023::read_input;
use nom::{
    IResult,
    character::complete::{u32 as parse_u32, multispace1, multispace0}, sequence::tuple, bytes::complete::tag, multi::{separated_list1, many1},
};


struct Card {
    id: u32,
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
            id,
            winning_numbers: winning_numbers.into_iter().collect(),
            my_numbers: my_numbers.into_iter().collect(),
        }))
    }

    pub fn count_winning(&self) -> u32 {
        self.winning_numbers.intersection(&self.my_numbers).count() as u32
    }
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("04");

    // first parse all cards
    let lines = input.lines();
    let mut cards = Vec::with_capacity(lines.size_hint().0);
    for line in input.lines() {
        let (_, card) = Card::from_line(line).unwrap();
        cards.push(card);
    }
    // compute winning values in reverse order
    let mut card_values = vec![0u32; cards.len()];
    for i in (0..cards.len()).rev() {
        let card = &cards[i];
        let win_count = card.count_winning();
        if win_count == 0 {
            card_values[i] = 1;
        } else {
            let transitive_card_count: u32 = card_values[i + 1..i + 1 + win_count as usize].iter().copied().sum();
            card_values[i] = transitive_card_count + 1;
        }
    }

    let sum: u32 = card_values.iter().sum();
    println!("Total time: {:?}", start_time.elapsed());
    println!("{}", sum);
}
