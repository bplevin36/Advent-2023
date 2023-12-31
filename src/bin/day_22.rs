use std::{time::Instant, collections::{HashSet, HashMap}, mem};

use aoc2023::read_input;
use nom::{IResult, sequence::{separated_pair, tuple}, character::complete::u32 as pu32, bytes::complete::tag, multi::separated_list1};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Brick {
    id: u32,
    start_coord: (u32, u32, u32),
    end_coord: (u32, u32, u32),
}

impl Brick {
    fn parse(mut id: u32) -> impl FnMut(&str) -> IResult<&str, Brick> {
        move |input: &str| {
            let (i, ((x1, _, y1, _, z1), (x2, _, y2, _, z2))) = separated_pair(
                tuple((pu32, tag(","), pu32, tag(","), pu32)),
                tag("~"),
                tuple((pu32, tag(","), pu32, tag(","), pu32))
            )(input)?;
            if x2 < x1 || y2 < y1 || z2 < z1 {
                panic!("Ordering of coords not consistent");
            }
            let brick = Brick {
                id,
                start_coord: (x1, y1, z1),
                end_coord: (x2, y2, z2),
            };
            id += 1;
            Ok((i, brick))
        }
    }

    fn top_z(&self) -> u32 {
        self.start_coord.2.max(self.end_coord.2)
    }

    fn bottom_z(&self) -> u32 {
        self.start_coord.2.min(self.end_coord.2)
    }

    fn footprint(&self) -> ((u32, u32), (u32, u32)) {
        ((self.start_coord.0, self.start_coord.1), (self.end_coord.0, self.end_coord.1))
    }

    fn footprint_intersects(&self, other: ((u32, u32), (u32, u32))) -> bool {
        let ((x1, y1), (x2, y2)) = self.footprint();
        let ((ox1, oy1), (ox2, oy2)) = other;

        if x1 <= ox2 && x2 >= ox1 && y1 <= oy2 && y2 >= oy1 {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Pile {
    bricks: Vec<Brick>,
}

impl Pile {
    fn parse(input: &str) -> Pile {
        let (_, bricks) = separated_list1(tag("\n"), Brick::parse(1))(input).unwrap();
        Pile {
            bricks
        }
    }

    fn settle_down(&mut self) {
        self.bricks.sort_unstable_by(|b1, b2| b2.bottom_z().cmp(&b1.bottom_z()));
        let mut processed: Vec<Brick> = Vec::new();

        while let Some(mut brick) = self.bricks.pop() {
            let footprint = brick.footprint();
            let fall_height = match processed.iter().copied()
                .filter(|b| b.footprint_intersects(footprint))
                .max_by_key(Brick::top_z)
            {
                Some(brick_to_land_on) => {
                    brick.bottom_z() - (brick_to_land_on.top_z() + 1)
                },
                None => { // fall all the way to the ground
                    brick.bottom_z() - 1
                },
            };
            brick.start_coord.2 -= fall_height;
            brick.end_coord.2 -= fall_height;
            processed.push(brick);
        }

        mem::swap(&mut self.bricks, &mut processed);
    }

    fn list_this_is_resting_on(&self, brick_idx: usize) -> HashSet<Brick> {
        let brick = self.bricks[brick_idx];
        let bottom_z = brick.bottom_z();
        let footprint = brick.footprint();

        let mut resting_on = HashSet::new();
        for other_brick in self.bricks[..brick_idx].iter().rev() {
            if other_brick.top_z() == bottom_z - 1 && other_brick.footprint_intersects(footprint) {
                resting_on.insert(*other_brick);
            }
        }
        resting_on
    }
}

fn main () {
    let start_time = Instant::now();
    let input = read_input("22");

    let mut pile = Pile::parse(&input);
    pile.settle_down();

    let mut brick_to_resting_on: HashMap<Brick, HashSet<Brick>> = HashMap::new();

    for brick_idx in (0..pile.bricks.len()).rev() {
        let brick = pile.bricks[brick_idx];
        let this_is_resting_on = pile.list_this_is_resting_on(brick_idx);
        brick_to_resting_on.insert(brick, this_is_resting_on);
    }

    let mut sum = 0;
    for brick_idx in 0..pile.bricks.len() {
        let brick = pile.bricks[brick_idx];
        let mut destroyed = HashSet::new();
        destroyed.insert(brick);
        for other_brick_idx in brick_idx + 1..pile.bricks.len() {
            let other_brick = pile.bricks[other_brick_idx];
            let other_brick_resting_on = &brick_to_resting_on[&other_brick];
            if other_brick_resting_on.len() > 0 && other_brick_resting_on.is_subset(&destroyed) {
                destroyed.insert(other_brick);
            }
        }
        destroyed.remove(&brick);  // we don't count the brick we chose to destroy to start the chain reaction
        sum += destroyed.len();
    }
    println!("{:?}", sum);
    println!("Total time: {:?}", start_time.elapsed());
}
