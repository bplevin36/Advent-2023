use std::fs;

pub fn read_input(day: &str) -> String {
    fs::read_to_string(format!("input/day_{}.txt", day)).unwrap()
}
