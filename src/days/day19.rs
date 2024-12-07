
use crate::Solution;
use std::time::{Duration, Instant};

pub struct Day19 {}

impl Solution for Day19 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = problem_name(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn problem_name(_data: &str) -> (String, String) {

    (String::new(), String::new())
}

