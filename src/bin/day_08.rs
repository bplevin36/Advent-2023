use std::{collections::HashMap, time::Instant};

use aoc2023::read_input;
use nom::{bytes::complete::tag, IResult, character::complete::{alphanumeric1, newline, multispace1}, sequence::{terminated, tuple}, multi::separated_list1};


fn parse_instructions_and_nodes(input: &str) -> IResult<&str, (&str, Vec<(&str, &str, &str)>)> {
    tuple((
        terminated(alphanumeric1, multispace1),
        separated_list1(newline,
            tuple((
                terminated(alphanumeric1, tag(" = (")),
                terminated(alphanumeric1, tag(", ")),
                terminated(alphanumeric1, tag(")"))
            )))
    ))(input)
}
pub fn main() {
    let start_time = Instant::now();
    let input = read_input("08");
    let (_, (instructions, nodes)) = parse_instructions_and_nodes(&input).unwrap();

    let node_map: HashMap<&str, (&str, &str)> = nodes.into_iter().map(|(id, l, r)| (id, (l, r))).collect();

    let mut current_node_id = "AAA";
    let mut num_steps = 0;
    loop {
        for instr in instructions.chars() {
            let current_node = node_map[current_node_id];
            if instr == 'L' {
                current_node_id = current_node.0;
            } else {
                current_node_id = current_node.1;
            }
            num_steps += 1;
            if current_node_id == "ZZZ" {
                println!("Total time: {:?}", start_time.elapsed());
                println!("{}", num_steps);
                return
            }
        }
    }
}
