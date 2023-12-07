use std::time::Instant;

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::u32 as parse_u32,
    sequence::{
        tuple, separated_pair
    },
    branch::alt,
    multi::separated_list1
};
use aoc2023::read_input;


#[derive(Default, Debug, Clone, Copy)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Set {
    pub fn from_set_str(set_str: &str) -> IResult<&str, Set> {
        let (i, color_vals) = separated_list1(
            tag(", "),
            separated_pair(
                parse_u32,
                tag(" "),
                alt((tag("red"), tag("green"), tag("blue")))
            )
        )(set_str)?;
        let mut constructing = Set::default();
        for (val, color) in color_vals {
            match color {
                "red" => constructing.red = val,
                "green" => constructing.green = val,
                "blue" => constructing.blue = val,
                _ => (),
            }
        }
        Ok((i, constructing))
    }

    pub fn min_set(self, other: Set) -> Set {
        Set {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    pub fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl Game {
    pub fn is_possible(&self) -> bool {
        for set in self.sets.iter() {
            if set.red > 12 {
                return false;
            } else if set.green > 13 {
                return false;
            } else if set.blue > 14 {
                return false;
            }
        }
        true
    }

    pub fn from_line(line: &str) -> IResult<&str, Game> {
        let (i, (_, id, _, sets)) = tuple((
            tag("Game "),
            parse_u32,
            tag(": "),
            separated_list1(tag("; "), Set::from_set_str),
        ))(line)?;
        Ok((i, Game {
            id,
            sets,
        }))
    }

    pub fn min_set(&self) -> Set {
        let mut set = self.sets[0];
        for other_set in &self.sets[1..] {
            set = set.min_set(*other_set);
        }
        set
    }
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("02");
    let start_compute_time = Instant::now();
    let mut sum = 0;
    for line in input.lines() {
        let (_, game) = Game::from_line(line).unwrap();
        sum += game.min_set().power();
    }
    println!("Total time: {:?}, compute time: {:?}", start_time.elapsed(), start_compute_time.elapsed());
    println!("{}", sum);

}
