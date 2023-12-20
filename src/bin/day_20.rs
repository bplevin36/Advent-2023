use std::{collections::{HashMap, VecDeque}, time::Instant};

use aoc2023::read_input;
use nom::{IResult, branch::alt, combinator::map, sequence::{preceded, separated_pair}, multi::separated_list1, character::complete::alpha1, bytes::complete::tag};

enum ModuleKind<'m> {
    FlipFlop{ on: bool },
    Conjuction{ last_inputs: HashMap<&'m str, bool>},
    Broadcaster,
}
struct Module<'m> {
    id: &'m str,
    kind: ModuleKind<'m>,
    outs: Vec<&'m str>,
}

impl<'m> Module<'m> {
    fn parse(input: &str) -> IResult<&str, Module> {
        let (i, module) = alt((
            map(preceded(tag("broadcaster -> "), separated_list1(tag(", "), alpha1)), |outs| Module{ id: "broadcaster", kind: ModuleKind::Broadcaster, outs }),
            map(preceded(tag("%"), separated_pair(alpha1, tag(" -> "), separated_list1(tag(", "), alpha1))), |(id, outs)| Module { id, kind: ModuleKind::FlipFlop { on: false }, outs }),
            map(preceded(tag("&"), separated_pair(alpha1, tag(" -> "), separated_list1(tag(", "), alpha1))), |(id, outs)| Module { id, kind: ModuleKind::Conjuction { last_inputs: HashMap::new()}, outs }),
        ))(input)?;
        Ok((i, module))
    }

    fn process_pulse(&mut self, pulse: bool, sender: &'m str) -> Vec<(&'m str, &'m str, bool)> {
        let mut send = None;
        let mut pending = Vec::new();
        match self.kind {
            ModuleKind::FlipFlop { ref mut on } => {
                if !pulse {
                    *on = !*on;
                    send = Some(*on);
                }
            },
            ModuleKind::Conjuction { ref mut last_inputs } => {
                *(last_inputs.get_mut(sender).unwrap()) = pulse;
                if last_inputs.values().all(|p| *p) {
                    send = Some(false);
                } else {
                    send = Some(true);
                }
            },
            ModuleKind::Broadcaster => {
                send = Some(pulse);
            },
        }
        if let Some(send_value) = send {
            for &destination in self.outs.iter() {
                pending.push((self.id, destination, send_value));
            }
        }
        pending
    }
}

fn parse_modules(input: &str) -> HashMap<&str, Module> {
    let (_, mut modules) = separated_list1(tag("\n"), Module::parse)(input).unwrap();
    // allocate a last input for all the inputs of conjunctions
    for i in 0..modules.len() {
        let (before, after) = modules.split_at_mut(i);
        let (module, after) = after.split_first_mut().unwrap();
        if let &mut Module { id, kind: ModuleKind::Conjuction { ref mut last_inputs }, outs: _ } = module {
            for other_module in before {
                if other_module.outs.contains(&id) {
                    last_inputs.insert(other_module.id, false);
                }
            }
            for other_module in after {
                if other_module.outs.contains(&id) {
                    last_inputs.insert(other_module.id, false);
                }
            }
        }
    }
    // convert to map
    let module_map = modules.into_iter().map(|m| (m.id, m)).collect();
    module_map
}

fn run_button_push(modules: &mut HashMap<&str, Module>, watched_modules: &mut HashMap<&str, usize>, iteration: usize) {
    let mut pending: VecDeque<(&str, &str, bool)> = VecDeque::from([("", "broadcaster", false)]);

    loop {
        let (sender, dest, pulse) = match pending.pop_front() {
            Some(t) => t,
            None => break,
        };
        if !pulse {
            if let Some(n) = watched_modules.get_mut(dest) {
                *n = iteration;
                if watched_modules.values().all(|&v| v != 0) {
                    return;
                }
            }
        }
        let module = match modules.get_mut(dest) {
            Some(m) => m,
            None => continue,
        };
        let next_pending = module.process_pulse(pulse, sender);
        pending.extend(next_pending);
    }

}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("20");

    let mut modules = parse_modules(&input);
    let mut watched_modules: HashMap<&str, usize> = HashMap::new();
    /* HACK: examination of the graph shows these node IDs determine the output
       and receive messages from loops */
    watched_modules.insert("tr", 0);
    watched_modules.insert("dr", 0);
    watched_modules.insert("xm", 0);
    watched_modules.insert("nh", 0);

    for num_pushes in 1..100_000 {
        run_button_push(&mut modules, &mut watched_modules, num_pushes);
        if watched_modules.values().all(|&v| v != 0) {
            let iteration = watched_modules.values().copied().reduce(lcm).unwrap();
            println!("{}", iteration);
            println!("Total time: {:#?}", start_time.elapsed());
            return;
        }
    }
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
