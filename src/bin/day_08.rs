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

// adapted from Wikipedia's description of Stein's Algorithm
pub fn gcd(mut u: usize, mut v: usize) -> usize {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

    // `|` is bitwise OR. `trailing_zeros` quickly counts a binary number's
    // trailing zeros, giving its prime factorization's exponent on two.
    let gcd_exponent_on_two = (u | v).trailing_zeros();

    // `>>=` divides the left by two to the power of the right, storing that in
    // the left variable. `u` divided by its prime factorization's power of two
    // turns it odd.
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    while u != v {
        if u < v {
            // Swap the variables' values with each other.
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    // `<<` multiplies the left by two to the power of the right.
    u << gcd_exponent_on_two
}

fn lcm(val1: usize, val2: usize) -> usize {
    let multiple = val1 * val2;
    multiple / gcd(val1, val2) as usize
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("08");
    let (_, (instructions, nodes)) = parse_instructions_and_nodes(&input).unwrap();

    let node_map: HashMap<&str, (&str, &str)> = nodes.into_iter().map(|(id, l, r)| (id, (l, r))).collect();

    let mut current_node_ids: Vec<&str> = node_map.keys().map(|k| *k).filter(|k| k.ends_with("A")).collect();
    let mut num_steps = 0;
    let mut starting_distance_to_z: Vec<usize> = vec![0; current_node_ids.len()];
    loop {
        for instr in instructions.as_bytes() {
            for current_node_id in current_node_ids.iter_mut() {
                let current_node = node_map[current_node_id];
                if *instr == b'L' {
                    *current_node_id = current_node.0;
                } else {
                    *current_node_id = current_node.1;
                }

            }
            num_steps += 1;
            for (idx, id) in current_node_ids.iter().enumerate() {
                if id.ends_with("Z") {
                    if starting_distance_to_z[idx] == 0 {
                        starting_distance_to_z[idx] = num_steps;
                        if !starting_distance_to_z.contains(&0) {
                            let total_lcm = starting_distance_to_z.into_iter().reduce(lcm).unwrap();
                            println!("Total time: {:?}", start_time.elapsed());
                            println!("{}", total_lcm);
                            return
                        }
                    }
                }

            }
        }
    }
}
