use std::{collections::HashMap, cmp::Ordering, time::Instant, ops::Range};

use aoc2023::read_input;
use nom::{IResult, bytes::complete::tag, character::complete::{u16 as parse_u16, alpha1, one_of}, sequence::{delimited, preceded, tuple}, branch::alt, combinator::map, multi::separated_list1};

struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    fn parse(input: &str) -> IResult<&str, Part> {
        let (i ,(x, m, a, s)) = delimited(
            tag("{"),
            tuple((
                delimited(tag("x="), parse_u16, tag(",")),
                delimited(tag("m="), parse_u16, tag(",")),
                delimited(tag("a="), parse_u16, tag(",")),
                preceded(tag("s="), parse_u16),
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
    Cmp{field: char, order: Ordering, value: u16, destination: &'i str},
    Redirect{workflow_name: &'i str},
}

impl<'i> Instr<'i> {
    fn parse(input: &'i str) -> IResult<&'i str, Instr<'i>> {
        alt((
            map(tag("A"), |_| Instr::Accept),
            map(tag("R"), |_| Instr::Reject),
            map(
                tuple((one_of("xmas"), one_of("<>"), parse_u16, tag(":"), alpha1)),
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

#[derive(Clone, Debug)]
struct FieldRange {
    x: Range<u16>,
    m: Range<u16>,
    a: Range<u16>,
    s: Range<u16>,
}

impl FieldRange {
    fn new() -> FieldRange {
        FieldRange { x: 1..4001, m: 1..4001, a: 1..4001, s: 1..4001 }
    }

    fn split(mut self, field: char, at: u16) -> (FieldRange, Option<FieldRange>) {
        let mut other = None;
        match field {
            'x' => {
                if self.x.contains(&at) {
                    let orig_x = self.x.clone();
                    let mut other_field_range = self.clone();
                    self.x = orig_x.start..at;
                    other_field_range.x = at..orig_x.end;
                    other = Some(other_field_range);
                }
            },
            'm' => {
                if self.m.contains(&at) {
                    let orig_m = self.m.clone();
                    let mut other_field_range = self.clone();
                    self.m = orig_m.start..at;
                    other_field_range.m = at..orig_m.end;
                    other = Some(other_field_range);
                }
            },
            'a' => {
                if self.a.contains(&at) {
                    let orig_a = self.a.clone();
                    let mut other_field_range = self.clone();
                    self.a = orig_a.start..at;
                    other_field_range.a = at..orig_a.end;
                    other = Some(other_field_range);
                }
            },
            's' => {
                if self.s.contains(&at) {
                    let orig_s = self.s.clone();
                    let mut other_field_range = self.clone();
                    self.s = orig_s.start..at;
                    other_field_range.s = at..orig_s.end;
                    other = Some(other_field_range);
                }
            },
            _ => panic!("Invalid"),
        }
        (self, other)
    }

    fn count_possible(&self) -> u64 {
        let mut count = 1u64;
        count *= (self.x.end).saturating_sub(self.x.start) as u64;
        count *= (self.m.end).saturating_sub(self.m.start) as u64;
        count *= (self.a.end).saturating_sub(self.a.start) as u64;
        count *= (self.s.end).saturating_sub(self.s.start) as u64;
        count
    }
}

fn run_range(workflows: &HashMap<&str, Vec<Instr>>, mut range: FieldRange, start_instruction: (&str, usize), accepting_ranges: &mut Vec<FieldRange>) {
    if start_instruction.0 == "A" {
        accepting_ranges.push(range);
        return;
    } else if start_instruction.0 == "R" {
        return;
    }
    for instruction in &workflows[start_instruction.0][start_instruction.1..] {
        match instruction {
            Instr::Accept => {
                accepting_ranges.push(range);
                return;
            },
            Instr::Reject => return,
            Instr::Cmp { field, order, value, destination } => {
                let at = match order {
                    Ordering::Less => *value,
                    Ordering::Greater => value + 1,
                    _ => panic!("Invalid"),
                };
                match range.split(*field, at) {
                    (smaller_range, Some(larger_range)) => {
                        match order {
                            Ordering::Less => {
                                range = larger_range;
                                run_range(workflows, smaller_range, (destination, 0), accepting_ranges);
                            },
                            Ordering::Greater => {
                                range = smaller_range;
                                run_range(workflows, larger_range, (destination, 0), accepting_ranges);
                            },
                            _ => panic!("Invalid"),
                        }
                    },
                    (unchanged_range, None) => {
                        range = unchanged_range;
                    }
                }
            },
            Instr::Redirect { workflow_name } => {
                run_range(workflows, range.clone(), (workflow_name, 0), accepting_ranges);
            },
        }
    }
}

pub fn main() {
    let start_time = Instant::now();
    let input = read_input("19");
    let (workflows, _) = parse_workflows_and_parts(&input);

    let mut accepting_ranges = Vec::new();

    run_range(&workflows, FieldRange::new(), ("in", 0), &mut accepting_ranges);

    let sum: u64 = accepting_ranges.iter().map(|r| r.count_possible()).sum();
    println!("{:#?}", sum);
    println!("Total time: {:?}", start_time.elapsed());
}
