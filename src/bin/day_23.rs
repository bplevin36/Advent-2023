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

    fn possible() -> &'static [Direction] {
        &[Direction::North, Direction::East, Direction::South, Direction::West]
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Path,
    Block,
    Slope(Direction),
}

#[derive(Clone)]
struct Path {
    tiles: Vec<(usize, usize)>,
    tile_set: HashSet<(usize, usize)>,
}

impl Path {
    fn from_tiles(tiles: Vec<(usize, usize)>) -> Path {
        Path {
            tile_set: tiles.iter().copied().collect(),
            tiles: tiles,
        }
    }
    fn add_coord(&mut self, coord: (usize, usize)) {
        self.tiles.push(coord);
        self.tile_set.insert(coord);
    }
}


fn main() {
    let start_time = Instant::now();
    let input = read_input("23");

    let grid: Vec<Vec<Tile>> = input.lines()
        .map(|l| {
            l.as_bytes().iter().map(|b| match b {
                b'.' => Tile::Path,
                b'#' => Tile::Block,
                b'>' => Tile::Slope(Direction::East),
                b'^' => Tile::Slope(Direction::North),
                b'<' => Tile::Slope(Direction::West),
                b'v' => Tile::Slope(Direction::South),
                _ => panic!("Invalid"),
            }).collect()
        }).collect();

    let start = (0, 1);
    let dest = (grid.len() - 1, grid.last().unwrap().len() - 2);

    let mut paths: Vec<Path> = vec![Path::from_tiles(vec![(start)])];
    let mut all_found = false;
    while !all_found {
        all_found = true;
        let mut path_idx = 0;
        while path_idx < paths.len() {
            if !paths[path_idx].tile_set.contains(&dest) {
                all_found = false;
                let tip_coord = paths[path_idx].tiles.last().copied().unwrap();
                let tip_tile = grid[tip_coord.0][tip_coord.1];
                let mut possible_directions: Vec<Direction> = Vec::new();
                if let Tile::Slope(direction) = tip_tile {
                    possible_directions.push(direction);
                } else {
                    possible_directions = Direction::possible().to_owned();
                }
                let next_coords: Vec<(usize, usize)> = possible_directions.into_iter().filter_map(|direction| {
                    let (row_delta, col_delta) = direction.to_coord_delta();
                    let next_coord = match (
                            tip_coord.0.checked_add_signed(row_delta as isize),
                            tip_coord.1.checked_add_signed(col_delta as isize)) {
                        (Some(row), Some(col)) => (row, col),
                        _ => return None,
                    };
                    if paths[path_idx].tile_set.contains(&next_coord) {
                        return None;
                    }
                    if let Tile::Block = grid[next_coord.0][next_coord.1] {
                        return None;
                    }
                    Some(next_coord)
                }).collect();
                if next_coords.len() == 0 {
                    // dead end
                    paths.remove(path_idx);
                    continue;
                }
                if next_coords.len() > 1 {
                    for next_coord in &next_coords[1..] {
                        let mut other_path = paths[path_idx].clone();
                        other_path.add_coord(*next_coord);
                        paths.push(other_path);
                    }
                }
                paths[path_idx].add_coord(next_coords[0]);
            }
            path_idx += 1;
        }
    }
    let max_len = paths.iter().max_by_key(|p| p.tiles.len()).unwrap().tiles.len() - 1;
    println!("{}", max_len);
    println!("Total time: {:?}", start_time.elapsed());
}
