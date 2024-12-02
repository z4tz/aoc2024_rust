pub mod days;

use std::fs;
use std::time::Duration;

pub fn aoc_reader(day: &i8) -> String {
    let filename = format!("inputs/day{day}.txt");
    let content = fs::read_to_string(&filename).expect(format!("Error reading file {filename}").as_str());
    content
}

pub trait Solution {
    fn timed_solution(&self, data: &str) -> (i32, i32, Duration);
}