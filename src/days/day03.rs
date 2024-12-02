use std::time::{Duration, Instant};
use crate::Solution;

pub struct Day03 {}

impl Solution for Day03 {
    fn timed_solution(&self, data: &str) -> (i32, i32, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = part1(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn part1(_data: &str) -> (i32, i32) {
    (1,2)
}