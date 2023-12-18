use std::time::Instant;

use aoc2023::read_input;

fn shoelace(coord1: (isize, isize), coord2: (isize, isize)) -> isize {
    let (x1, y1) = coord1;
    let (x2, y2) = coord2;
    x1 * y2 - y1 * x2
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("18");

    let instructions = input.lines().map(|line| {
        let mut fields = line.split(" ");
        let _color = fields.next().unwrap();
        let _ = fields.next().unwrap().parse::<isize>().unwrap();
        let mut color_field = fields.next().unwrap();
        color_field = &color_field[2..color_field.len() - 1];
        let distance = match isize::from_str_radix(&color_field[0..5], 16) {
            Ok(d) => d,
            Err(_) => panic!("Could not parse {}", &color_field[0..5]),
        };
        let direction = match color_field.as_bytes()[5] {
            b'0' => "R",
            b'1' => "D",
            b'2' => "L",
            b'3' => "U",
            _ => panic!("Invalid"),
        };
        (direction, distance)
    }).collect::<Vec<(&str, isize)>>();

    let (mut row, mut col) = (0isize, 0isize);

    let mut double_area = 0isize;

    for i in 0..instructions.len() {
        let &(direction, distance) = &instructions[i];
        let (prev_row, prev_col) = (row, col);
        match direction {
            "U" => {
                row -= distance;
            },
            "D" => {
                row += distance;
            },
            "R" => {
                col += distance;
            },
            "L" => {
                col -= distance;
            },
            _ => panic!("Invalid"),
        }
        double_area += shoelace((prev_col, prev_row), (col, row));
        // pick's theorem
        double_area += distance;
    }

    println!("{}", double_area / 2 + 1);  // add one from start point
    println!("Total time: {:?}", start_time.elapsed());
}
