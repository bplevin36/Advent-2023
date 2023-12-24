use std::{collections::{HashSet, HashMap}, time::Instant};

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
    fn invert(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
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

fn follow_to_intersection(
    grid: &Vec<Vec<Tile>>, start: (usize, usize), mut direction: Direction
    // (intersection coord, vec of next coords, steps taken)
) -> ((usize, usize), Vec<((usize, usize), Direction)>, usize) {
    let mut coord = start;
    let mut steps = 1;
    loop {
        let next_coords: Vec<((usize, usize), Direction)> = Direction::possible().iter().filter_map(|&next_direction| {
            if next_direction == direction.invert() {
                return None;
            }
            let (row_delta, col_delta) = next_direction.to_coord_delta();
            let next_coord = match (
                    coord.0.checked_add_signed(row_delta as isize),
                    coord.1.checked_add_signed(col_delta as isize)) {
                (Some(row), Some(col)) => (row, col),
                _ => return None,
            };
            if next_coord.0 >= grid.len() || next_coord.1 >= grid[0].len() {
                return None;
            }
            if let Tile::Block = grid[next_coord.0][next_coord.1] {
                return None;
            }
            Some((next_coord, next_direction))
        }).collect();
        if next_coords.len() > 1 || next_coords.len() == 0 {
            return (coord, next_coords, steps);
        } else {
            coord = next_coords[0].0;
            direction = next_coords[0].1;
            steps += 1;
        }
    }
}

#[derive(Debug)]
struct Node {
    _coord: (usize, usize),
    // (coord, distance)
    edges: Vec<((usize, usize), usize)>,
}

impl Node {
    fn from_coord(coord: (usize, usize)) -> Node {
        Node {
            _coord: coord,
            edges: vec![],
        }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<(usize, usize), Node>,
}

impl Graph {
    fn from_grid(grid: &Vec<Vec<Tile>>) -> Graph {
        let start = (0, 1);
        let dest = (grid.len() - 1, grid.last().unwrap().len() - 2);

        let mut intersections: Vec<((usize, usize), Vec<((usize, usize), Direction)>)> = vec![];
        intersections.push((start, vec![((start.0 + 1, start.1), Direction::South)]));
        intersections.push((dest, vec![((dest.0 - 1, dest.1), Direction::North)]));
        for row_idx in 1..(grid.len() - 1) {
            let row = &grid[row_idx];
            for col_idx in 1..(row.len() - 1) {
                let tile = row[col_idx];
                if let Tile::Block = tile {
                    continue;
                }
                // non-block neighbors
                let neighbors: Vec<((usize, usize), Direction)> = Direction::possible().into_iter().filter_map(|d| {
                    let (row_delta, col_delta) = d.to_coord_delta();
                    let neighbor_row = row_idx.wrapping_add_signed(row_delta as isize);
                    let neighbor_col = col_idx.wrapping_add_signed(col_delta as isize);
                    if let Tile::Block = grid[neighbor_row][neighbor_col] {
                        return None;
                    }
                    Some(((neighbor_row, neighbor_col), *d))
                }).collect();
                if neighbors.len() > 2 {
                    intersections.push(((row_idx, col_idx), neighbors));
                }
            }
        }
        let mut nodes: HashMap<(usize, usize), Node> = HashMap::new();
        for (intersection, branches) in intersections {
            let mut node = Node::from_coord(intersection);

            for (branch_start, direction) in branches {
                let (next_intersection, _next_coords, steps) = follow_to_intersection(&grid, branch_start, direction);
                node.edges.push((next_intersection, steps));
            }
            nodes.insert(intersection, node);
        }
        Graph {
            nodes,
        }
    }
}

fn find_max_path(graph: &Graph, start: (usize, usize), dest: (usize, usize), mut discovered: HashSet<(usize, usize)>) -> Option<usize> {
    if start == dest {
        return Some(0);
    }
    discovered.insert(start);
    let node = &graph.nodes[&start];
    let mut max_path: Option<usize> = None;
    for (other_coord, distance) in node.edges.iter() {
        if !discovered.contains(other_coord) {
            match find_max_path(graph, *other_coord, dest, discovered.clone()) {
                None => (),
                Some(path_length) => {
                    max_path = Some(max_path.unwrap_or_default().max(distance + path_length));
                }
            }
        }
    }
    max_path
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
    // build graph from grid
    let graph = Graph::from_grid(&grid);

    let discovered = HashSet::new();
    let max_len = find_max_path(&graph, start, dest, discovered).unwrap();
    println!("{}", max_len);
    println!("Total time: {:?}", start_time.elapsed());
}
