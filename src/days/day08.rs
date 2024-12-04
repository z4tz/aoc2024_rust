
use crate::Solution;
use std::time::{Duration, Instant};

pub struct Day08 {}

impl Solution for Day08 {
    fn timed_solution(&self, data: &str) -> (i32, i32, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = problem_name(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn problem_name(_data: &str) -> (i32, i32) {

    (1,2)
}

