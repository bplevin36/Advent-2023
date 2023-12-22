use std::{mem, time::Instant, collections::HashMap};

use aoc2023::read_input;
use nom::{IResult, multi::{separated_list1, many1}, character::complete::{newline, one_of}, combinator::map};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

    fn _to_char(self) -> char {
        match self {
            Plot::Garden => '.',
            Plot::Rock => '#',
            Plot::Reached => 'O',
        }
    }

    fn set_reached_if_not_rock(&mut self) -> bool {
        match self {
            Plot::Rock => false,
            _ => {
                *self = Plot::Reached;
                true
            },
        }
    }
}

fn parse_map(input: &str) -> IResult<&str, Vec<Vec<Plot>>> {
    separated_list1(newline, many1(map(one_of(".#S"), |c| Plot::from_char(c))))(input)
}

fn reset_reached(next_grids: &mut HashMap<(i32, i32), Vec<Vec<Plot>>>) {
    for (_key, grid) in next_grids.iter_mut() {
        for row in grid.iter_mut() {
            for plot in row.iter_mut() {
                if *plot == Plot::Reached {
                    *plot = Plot::Garden;
                }
            }
        }
    }
}

fn _print_grids(grids: &HashMap<(i32, i32), Vec<Vec<Plot>>>) {
    let mut keys: Vec<(i32, i32)> = grids.keys().copied().collect();
    keys.sort();
    for key in keys {
        let grid = &grids[&key];
        println!("{:?}", key);
        for row in grid.iter() {
            for plot in row.iter() {
                print!("{}", plot._to_char());
            }
            print!("\n");
        }
        println!("");
    }
}

fn run_simulation(
    steps: usize,
    mut curr_grids: &mut HashMap<(i32, i32), Vec<Vec<Plot>>>,
) {
    let mut orig_grids = curr_grids.clone();
    reset_reached(&mut orig_grids);
    let orig_empty = &orig_grids[&(0, 0)];
    let clone_empty = || {
        orig_empty.clone()
    };
    let mut next_grids = curr_grids.clone();


    for _ in 0..steps {
        reset_reached(&mut next_grids);
        // swap buffer means next grid may need to have some keys added
        for key in curr_grids.keys() {
            if !next_grids.contains_key(key) {
                next_grids.insert(*key, orig_empty.clone());
            }
        }
        for grid_coord in curr_grids.keys() {
            let curr_grid = &curr_grids[&grid_coord];
            for row_idx in 0..curr_grid.len() {
                let row = &curr_grid[row_idx];
                for col_idx in 0..row.len() {
                    let plot = &row[col_idx];
                    if let Plot::Reached = plot {
                        if col_idx > 0 {
                            next_grids.get_mut(&grid_coord).unwrap()[row_idx][col_idx - 1].set_reached_if_not_rock();
                        } else {
                            let neighbor_idx = (grid_coord.0, grid_coord.1 - 1);
                            let neighbor_grid = next_grids.entry(neighbor_idx).or_insert_with(clone_empty);
                            neighbor_grid[row_idx].last_mut().unwrap().set_reached_if_not_rock();
                        }
                        if col_idx + 1 < curr_grid[row_idx].len() {
                            next_grids.get_mut(&grid_coord).unwrap()[row_idx][col_idx + 1].set_reached_if_not_rock();
                        } else {
                            let neighbor_idx = (grid_coord.0, grid_coord.1 + 1);
                            let neighbor_grid = next_grids.entry(neighbor_idx).or_insert_with(clone_empty);
                            neighbor_grid[row_idx][0].set_reached_if_not_rock();
                        }
                        if row_idx > 0 {
                            next_grids.get_mut(&grid_coord).unwrap()[row_idx - 1][col_idx].set_reached_if_not_rock();
                        } else {
                            let neighbor_idx = (grid_coord.0 - 1, grid_coord.1);
                            let neighbor_grid = next_grids.entry(neighbor_idx).or_insert_with(clone_empty);
                            neighbor_grid.last_mut().unwrap()[col_idx].set_reached_if_not_rock();
                        }
                        if row_idx + 1 < curr_grid.len() {
                            next_grids.get_mut(&grid_coord).unwrap()[row_idx + 1][col_idx].set_reached_if_not_rock();
                        } else {
                            let neighbor_idx = (grid_coord.0 + 1, grid_coord.1);
                            let neighbor_grid = next_grids.entry(neighbor_idx).or_insert_with(clone_empty);
                            neighbor_grid[0][col_idx].set_reached_if_not_rock();
                        }
                    }
                }
            }
        }
        mem::swap(&mut next_grids, &mut curr_grids);
    }
}

const FIRST_CYCLE: usize = 65;
const TARGET_STEPS: usize = 26_501_365;
const NUM_CYCLES: usize = (TARGET_STEPS - FIRST_CYCLE) / 131;
const _: () = assert!(NUM_CYCLES % 2 == 0, "cycles must be even");

fn main() {
    let start_time = Instant::now();
    let input = read_input("21");

    let (_, curr_map) = parse_map(&input).unwrap();

    let mut curr_grids = HashMap::new();

    curr_grids.insert((0, 0), curr_map);

    // just enough to get us to 5x5 macro grid
    run_simulation(65 + (131 * 2), &mut curr_grids);

    let mut known_counts = HashMap::new();
    for (key, grid) in curr_grids.iter() {
        known_counts.insert(key, grid.iter().map(|v| v.iter()).flatten().filter(|&&p| p == Plot::Reached).count());
    }

    let mut total = 0usize;
    // first, add corners
    total += known_counts[&(2, 0)] + known_counts[&(0, -2)] + known_counts[&(-2, 0)] + known_counts[&(0, 2)];

    // now add top left and top right
    total += (known_counts[&(-2, -1)] * NUM_CYCLES) + (known_counts[&(-1, -1)] * (NUM_CYCLES - 1));
    total += (known_counts[&(-2, 1)] * NUM_CYCLES) + (known_counts[&(-1, 1)] * (NUM_CYCLES - 1));

    // for interior grids, there are two counts
    let even_interior = known_counts[&(0, 0)];
    let odd_interior = known_counts[&(0, 1)];
    // add top center and bottom center
    total += odd_interior;
    total += odd_interior;
    // add top half interior squares excluding middle row; then double it to get bottom as well
    let mut top_half_count = 0;
    for cycle in 1..NUM_CYCLES - 1 {
        let row_length = 1 + (cycle * 2);
        let even_count = row_length / 2;
        let odd_count = (row_length / 2) + 1;
        top_half_count += even_count * even_interior;
        top_half_count += odd_count * odd_interior;
    }
    total += top_half_count * 2;
    // add middle row
    let row_length = NUM_CYCLES * 2 - 1;
    total += (row_length / 2) * even_interior;
    total += ((row_length / 2) + 1) * odd_interior;
    // add bottom left and bottom right
    total += (known_counts[&(2, -1)] * NUM_CYCLES) + (known_counts[&(1, -1)] * (NUM_CYCLES - 1));
    total += (known_counts[&(2, 1)] * NUM_CYCLES) + (known_counts[&(1, 1)] * (NUM_CYCLES - 1));
    println!("{:#?}", total);
    println!("Total time: {:?}", start_time.elapsed());
}
