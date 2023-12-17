use std::{collections::HashSet, time::Instant};

use aoc2023::read_input;


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

    fn split(self) -> [Self; 2] {
        match self {
            Direction::North | Direction::South=> [Direction::East, Direction::West],
            Direction::East | Direction::West => [Direction::North, Direction::South],
        }
    }

    fn reflect(self, byte: u8) -> Direction {
        match (self, byte) {
            (Direction::East, b'/') | (Direction::West, b'\\') => Direction::North,
            (Direction::West, b'/') | (Direction::East, b'\\') => Direction::South,
            (Direction::North, b'/') | (Direction::South, b'\\') => Direction::East,
            (Direction::South, b'/') | (Direction::North, b'\\') => Direction::West,
            _ => panic!("Invalid"),
        }
    }
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("16");
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let mut energized: HashSet<(usize, usize)> = HashSet::new();
    let mut directions_passed: HashSet<(usize, usize, Direction)> = HashSet::new();

    let mut max_energized = 0;
    for i in 0..lines[0].len() {
        energized.clear();
        directions_passed.clear();
        project_beam(&lines, &mut energized, &mut directions_passed, (0, i), Direction::South);
        max_energized = max_energized.max(energized.iter().count());

        energized.clear();
        directions_passed.clear();
        project_beam(&lines, &mut energized, &mut directions_passed, (lines.len() - 1, i), Direction::North);
        max_energized = max_energized.max(energized.iter().count());
    }

    for i in 0..lines.len() {
        energized.clear();
        directions_passed.clear();
        project_beam(&lines, &mut energized, &mut directions_passed, (i, 0), Direction::East);
        max_energized = max_energized.max(energized.iter().count());

        energized.clear();
        directions_passed.clear();
        project_beam(&lines, &mut energized, &mut directions_passed, (i, lines[i].len() - 1), Direction::West);
        max_energized = max_energized.max(energized.iter().count());
    }

    println!("{}", max_energized);
    println!("Total time: {:?}", start_time.elapsed());
}

fn project_beam(
    lines: &[&[u8]],
    energized: &mut HashSet<(usize, usize)>,
    directions_passed: &mut HashSet<(usize, usize, Direction)>,
    start_position: (usize, usize),
    start_direction: Direction,
) {
    // recursively fill out the energized cells
    let mut position = start_position;
    let mut direction = start_direction;

    loop {
        if directions_passed.contains(&(position.0, position.1, direction)) {
            // hit a cycle; terminate
            return;
        }
        energized.insert(position);
        directions_passed.insert((position.0, position.1, direction));

        match (lines[position.0][position.1], direction){
            (b'.', _) | (b'-', Direction::West | Direction::East) | (b'|', Direction::North | Direction::South) => {
                // empty or passing through splitter
            },
            (b'-', Direction::North | Direction::South) | (b'|', Direction::East | Direction::West) => {
                // horizontal or vertical split
                let [direction1, direction2] = direction.split();
                if let Some(next_position1) = next_position(lines, position, direction1) {
                    project_beam(lines, energized, directions_passed, next_position1, direction1);
                }
                direction = direction2;
            },
            (reflector @ (b'\\' | b'/'), _) => {
                // single reflection
                direction = direction.reflect(reflector);
            },
            _ => panic!("Invalid"),
        }
        if let Some(valid_next_position) = next_position(lines, position, direction) {
            position = valid_next_position;
        } else {
            return;
        }
    }
}

fn next_position(lines: &[&[u8]], position: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    let (delta_line, delta_col) = direction.to_coord_delta();
    match (position.0.checked_add_signed(delta_line as isize), position.1.checked_add_signed(delta_col as isize)) {
        (None, _) | (_, None) => None,
        (Some(next_line), Some(next_col)) => {
            if next_line < lines.len() && next_col < lines[next_line].len() {
                Some((next_line, next_col))
            } else {
                None
            }
        },
    }
}
