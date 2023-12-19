use std::{collections::HashMap, cmp::Ordering, time::Instant};

use aoc2023::read_input;
use nom::{IResult, bytes::complete::tag, character::complete::{u32 as parse_u32, alpha1, one_of}, sequence::{delimited, preceded, tuple}, branch::alt, combinator::map, multi::separated_list1};

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn parse(input: &str) -> IResult<&str, Part> {
        let (i ,(x, m, a, s)) = delimited(
            tag("{"),
            tuple((
                delimited(tag("x="), parse_u32, tag(",")),
                delimited(tag("m="), parse_u32, tag(",")),
                delimited(tag("a="), parse_u32, tag(",")),
                preceded(tag("s="), parse_u32),
            )),
            tag("}")
        )(input)?;
        Ok((i, Part{
            x,
            m,
            a,
            s,
        }))
    }
}

enum Instr<'i> {
    Accept,
    Reject,
    Cmp{field: char, order: Ordering, value: u32, destination: &'i str},
    Redirect{workflow_name: &'i str},
}

impl<'i> Instr<'i> {
    fn parse(input: &'i str) -> IResult<&'i str, Instr<'i>> {
        alt((
            map(tag("A"), |_| Instr::Accept),
            map(tag("R"), |_| Instr::Reject),
            map(
                tuple((one_of("xmas"), one_of("<>"), parse_u32, tag(":"), alpha1)),
                |(field, cmp, value, _, destination)| Instr::Cmp {
                    field: field,
                    order: if cmp == '<' {Ordering::Less} else {Ordering::Greater},
                    value,
                    destination,
                }),
            map(alpha1, |workflow_name| Instr::Redirect { workflow_name }),
        ))(input)
    }
}

fn parse_workflows_and_parts(input: &str) -> (HashMap<&str, Vec<Instr>>, Vec<Part>) {
    let mut lines = input.lines();
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();
    loop {
        let line = match lines.next() {
            None => break,
            Some("") => break,
            Some(l) => l,
        };
        let (_, (name, instructions)) = tuple((
            alpha1, delimited(tag("{"), separated_list1(tag(","), Instr::parse), tag("}"))
        ))(line).unwrap();
        workflows.insert(name, instructions);
    }
    for line in lines {
        let (_, part) = Part::parse(line).unwrap();
        parts.push(part);
    }
    (workflows, parts)
}

fn run_workflows(part: &Part, workflows: &HashMap<&str, Vec<Instr>>) -> u32 {
    let mut workflow = &workflows["in"];
    loop {
        for instruction in workflow {
            match instruction {
                Instr::Accept => {
                    return part.x + part.m + part.a + part.s;
                },
                Instr::Reject => return 0,
                Instr::Cmp { field, order, value, destination } => {
                    let our_field = match field {
                        'x' => part.x,
                        'm' => part.m,
                        'a' => part.a,
                        's' => part.s,
                        _ => panic!("Invalid"),
                    };
                    if our_field.cmp(value) == *order {
                        match *destination {
                            "A" => return part.x + part.m + part.a + part.s,
                            "R" => return 0,
                            s => {
                                workflow = &workflows[s];
                                break;
                            },
                        }
                    }
                },
                Instr::Redirect { workflow_name } => {
                    workflow = &workflows[workflow_name];
                    break;
                },
            }
        }
    }
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("19");
    let (workflows, parts) = parse_workflows_and_parts(&input);

    let mut sum = 0;
    for part in parts.iter() {
        sum += run_workflows(part, &workflows);
    }

    println!("{}", sum);
    println!("Total time: {:?}", start_time.elapsed());
}
