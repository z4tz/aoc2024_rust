use crate::Solution;
use std::time::{Duration, Instant};
use std::collections::HashMap;

pub struct Day11 {}

impl Solution for Day11 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = problem_name(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn problem_name(data: &str) -> (String, String) {
    let mut stones:HashMap<i64, i64> = HashMap::new();

    for number in data.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()) {
        *stones.entry(number).or_insert(0) += 1;
    }

    let mut score1 = 0;
    let mut score2 = 0;
    for i in 0.. 75 {
        let mut new_stones = HashMap::new();

        for (value, count) in stones {
            if value == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if value.to_string().len() % 2 == 0 {
                let stonestring = value.to_string();
                let stoneparts = stonestring.split_at(stonestring.len() / 2);
                *new_stones.entry(stoneparts.0.parse::<i64>().unwrap()).or_insert(0) += count;
                *new_stones.entry(stoneparts.1.parse::<i64>().unwrap()).or_insert(0) += count;
            } else {
                *new_stones.entry(value * 2024).or_insert(0) += count;
            }
        }
        stones = new_stones;
        if i == 24 { score1 = stones.values().sum::<i64>()};
        if i == 74 { score2 = stones.values().sum::<i64>()};
    }

    (score1.to_string(), score2.to_string())
}


