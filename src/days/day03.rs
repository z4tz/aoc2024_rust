use std::time::{Duration, Instant};
use crate::Solution;
use regex::Regex;

pub struct Day03 {}

impl Solution for Day03 {
    fn timed_solution(&self, data: &str) -> (i32, i32, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = corrupted_multiplications(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn corrupted_multiplications(data: &str) -> (i32, i32) {

    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let total = mul_re.captures_iter(&data)
        .map(|c| c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap())
        .sum::<i32>();

    //Part 2
    let long_data = format!("do(){:?}don't()", data);  //starts with enable and ends with disable
    let splitter_re = Regex::new(r"do\(\)(.*?)don't\(\)").unwrap(); //pick out each enabled part
    let total2 = splitter_re.captures_iter(&long_data)
        .map(|c| c[1].to_string())
        .map(|s| mul_re.captures_iter(&s)
            .map(|c| c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap())
            .sum::<i32>())
        .sum::<i32>();

    (total,total2)
}