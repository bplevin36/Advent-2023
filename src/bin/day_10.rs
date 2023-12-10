use std::{time::Instant, fmt::Display};

use aoc2023::read_input;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn invert(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn to_coord_delta(self) -> (i8, i8) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }

    fn iter() -> impl Iterator<Item = Direction> {
        [Direction::North, Direction::East, Direction::South, Direction::West].iter().copied()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Pipe {
    None,
    Start,
    Vert,
    Horiz,
    NE,
    NW,
    SE,
    SW,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let byte = match self {
            Pipe::None => b'.',
            Pipe::Start => b'S',
            Pipe::Vert => b'|',
            Pipe::Horiz => b'-',
            Pipe::NE => b'L',
            Pipe::NW => b'J',
            Pipe::SE => b'F',
            Pipe::SW => b'7',
        };
        write!(f, "{}", byte as char)
    }
}

impl Pipe {
    fn from_byte(byte: u8) -> Pipe {
        match byte {
            b'.' => Pipe::None,
            b'S' => Pipe::Start,
            b'|' => Pipe::Vert,
            b'-' => Pipe::Horiz,
            b'L' => Pipe::NE,
            b'J' => Pipe::NW,
            b'7' => Pipe::SW,
            b'F' => Pipe::SE,
            _ => panic!("Invalid pipe value"),
        }
    }

    fn entrances(self) -> &'static [Direction] {
        match self {
            Pipe::None => &[],
            Pipe::Start => &[Direction::North, Direction::South, Direction::East, Direction::West],
            Pipe::Vert => &[Direction::North, Direction::South],
            Pipe::Horiz => &[Direction::East, Direction::West],
            Pipe::NE => &[Direction::North, Direction::East],
            Pipe::NW => &[Direction::North, Direction::West],
            Pipe::SE => &[Direction::South, Direction::East],
            Pipe::SW => &[Direction::South, Direction::West],
        }
    }

    fn next_entered_from(self, from: Direction) -> Direction {
        for &entrance in self.entrances() {
            if entrance != from {
                return entrance;
            }
        }
        unreachable!("No exit found");
    }

    fn has_entrance(self, direction: Direction) -> bool {
        self.entrances().contains(&direction)
    }
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("10");

    let lines: Vec<Vec<Pipe>> = input.lines()
        .map(|l| l.as_bytes().iter().copied().map(Pipe::from_byte).collect()).collect();

    let mut start = (0, 0);
    for (line_idx, line) in lines.iter().enumerate() {
        for (tile_idx, &tile) in line.iter().enumerate() {
            if tile == Pipe::Start {
                start = (line_idx, tile_idx);
                break;
            }
        }
    }

    // allocate a vec for the grid with all pipes not part of the loop removed
    let mut cleaned_lines: Vec<Vec<Pipe>> = Vec::with_capacity(lines.len());
    for line in lines.iter() {
        cleaned_lines.push(vec![Pipe::None; line.len()]);
    }
    cleaned_lines[start.0][start.1] = Pipe::NE;  // known from examining input

    let mut current_coord = start;
    let mut current_pipe = Pipe::Start;
    let mut came_from = Direction::South;
    // find next tile after start
    for direction in Direction::iter() {
        let (line_delta, col_delta) = direction.to_coord_delta();
        let adj_line = start.0.checked_add_signed(line_delta as isize);
        let adj_col = start.1.checked_add_signed(col_delta as isize);
        // underflow and edge checking, even though given input has start in middle
        match (adj_line, adj_col) {
            (None, _) => continue,
            (Some(adj_line_idx), _) if adj_line_idx >= lines.len() => continue,
            (Some(_), None) => continue,
            (Some(adj_line_idx), Some(adj_col_idx)) if adj_col_idx >= lines[adj_line_idx].len() => continue,
            (Some(adj_line_idx), Some(adj_col_idx)) => {
                let adj_pipe = lines[adj_line_idx][adj_col_idx];
                if adj_pipe.has_entrance(direction.invert()) {
                    current_coord = (adj_line_idx, adj_col_idx);
                    current_pipe = adj_pipe;
                    came_from = direction.invert();
                    cleaned_lines[current_coord.0][current_coord.1] = current_pipe;
                    break;
                }
            },
        }
    }
    if current_coord == start {
        panic!("Failed to find next tile")
    }

    loop {
        let direction = current_pipe.next_entered_from(came_from);
        let (line_delta, col_delta) = direction.to_coord_delta();

        let adj_line = current_coord.0.checked_add_signed(line_delta as isize).unwrap();
        let adj_col = current_coord.1.checked_add_signed(col_delta as isize).unwrap();
        let adj_pipe = lines[adj_line][adj_col];
        if !adj_pipe.has_entrance(direction.invert()) {
            panic!("Pipe {:?} at {:?} could not be entered going {:?} from {:?}", adj_pipe, (adj_line, adj_col), direction, current_coord);
        }
        current_coord = (adj_line, adj_col);
        current_pipe = adj_pipe;
        came_from = direction.invert();

        if current_coord == start {
            break;
        }
        cleaned_lines[current_coord.0][current_coord.1] = current_pipe;
    }

    // count points in cleaned grid that are inside loop
    let mut tiles_inside = 0;
    for line in cleaned_lines.iter_mut() {
        let mut times_crossed = 0;
        let mut from_south = false;
        let mut from_north = false;
        for pipe in line.iter_mut().rev() {
            match pipe {
                Pipe::None => {
                    if times_crossed % 2 == 1 {
                        tiles_inside += 1;
                    }
                },
                Pipe::Vert => {
                    times_crossed += 1;
                },
                Pipe::NW | Pipe::NE => {
                    if from_south {
                        times_crossed += 1;
                        from_south = false;
                    } else {
                        from_north = !from_north;
                    }
                },
                Pipe::SW | Pipe::SE => {
                    if from_north {
                        times_crossed += 1;
                        from_north = false;
                    } else {
                        from_south = !from_south;
                    }
                },
                _ => (),
            }
        }
    }

    println!("Total time: {:?}", start_time.elapsed());
    println!("{}", tiles_inside);
}
