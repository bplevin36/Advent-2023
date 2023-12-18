use std::{collections::{HashMap, HashSet}, cmp::{Ordering, Reverse}, time::Instant};

use aoc2023::read_input;
use priority_queue::PriorityQueue;


const MAX_RUN: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_coord_delta(self) -> (i8, i8) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    fn invert(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn iter() -> impl Iterator<Item = Direction> {
        [Direction::North, Direction::East, Direction::South, Direction::West].iter().copied()
    }
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("17");
    let lines: Vec<&str> = input.lines().collect();

    let mut nodes: HashMap<(isize, isize, Direction, u8), u8> = HashMap::new();
    let mut distance_to: HashMap<(isize, isize, Direction, u8), usize> = HashMap::new();

    // add nodes to graph; creating a duplicate for every run length
    for (line_idx, line) in lines.iter().enumerate() {
        for (col_idx, byte) in line.chars().enumerate() {
            let weight = byte.to_digit(10).unwrap() as u8;
            for direction in Direction::iter() {
                for straight_len in 1..=MAX_RUN {
                    let node = (line_idx as isize, col_idx as isize, direction, straight_len);
                    nodes.insert(node, weight);
                    distance_to.insert(node, usize::MAX);
                }
            }
        }
    }

    let end_coords = (lines.len() as isize - 1, lines[lines.len() - 1].len() as isize - 1);
    // run Djikstra's
    let mut queue: PriorityQueue<(isize, isize, Direction, u8), Reverse<usize>> = PriorityQueue::new();
    let mut current_node = (0, 0, Direction::East, 0);
    distance_to.insert(current_node, 0);
    queue.push(current_node, Reverse(0));
    loop {
        let (closest, Reverse(closest_dist)) = match queue.pop() {
            None => break,
            Some(x) => x,
        };
        current_node = closest;
        let old_distance = distance_to[&current_node];
        if closest_dist > old_distance {
            continue;
        }

        for (neighbor, weight) in Direction::iter()
            .filter_map(|d| {
                if d == current_node.2.invert() {
                    return None;
                }
                let delta = d.to_coord_delta();
                let next_0 = closest.0 + delta.0 as isize;
                let next_1 = closest.1 + delta.1 as isize;
                let run_length = if d == closest.2 { closest.3 + 1 } else { 1 };
                let possible_node = (next_0, next_1, d, run_length);
                match nodes.get(&possible_node) {
                    None => None,
                    Some(weight) => Some((possible_node, weight)),
                }
            })
        {
            let distance_to_neighbor = closest_dist + *weight as usize;
            let old_distance_to_neighbor = distance_to[&neighbor];
            if distance_to_neighbor < old_distance_to_neighbor {
                distance_to.insert(neighbor, distance_to_neighbor);
                queue.push(neighbor, Reverse(distance_to_neighbor));
            }
        }
    }

    // find the duplicate of the destination that has the shortest distance
    let mut shortest = usize::MAX;
    for direction in Direction::iter() {
        for run_length in 1..=MAX_RUN {
            let dist = distance_to[&(end_coords.0, end_coords.1, direction, run_length)];
            shortest = shortest.min(dist);
        }
    }

    println!("{}", shortest);
    println!("Total time: {:?}", start_time.elapsed());
}
