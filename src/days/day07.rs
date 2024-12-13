use crate::Solution;
use std::time::{Duration, Instant};
use itertools::{repeat_n, Itertools};

pub struct Day07 {}

impl Solution for Day07 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = calibration_results(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn calibration_results(data: &str) -> (String, String) {
    let multiply = |a:i64, b:i64| -> i64 {a*b};
    let add = |a:i64, b:i64| -> i64 {a+b};
    let concat = |a:i64, b:i64| -> i64 {i64::pow(10, b.checked_ilog10().unwrap() + 1) * a + b};
    let operators_part1 = vec![multiply, add];
    let operators_part2 = vec![multiply, add, concat];

    let mut calibration1 = 0;
    let mut calibration2 = 0;
    for line in data.lines() {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let result = parts[0].parse::<i64>().unwrap();
        let numbers= parts[1].split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        if valid_calibration(&operators_part1, result, &numbers) {
            calibration1 += result;
        }
        if valid_calibration(&operators_part2, result, &numbers) {
            calibration2 += result;
        }
    }
    (calibration1.to_string(),calibration2.to_string())
}
//Todo: replace with recursive function to avoid repeating calculations?
fn valid_calibration(operators:  &Vec<fn(i64, i64) -> i64>, result: i64, numbers: &Vec<i64>) -> bool{
    for operator_combination in repeat_n(operators, numbers.len()).multi_cartesian_product() {
        let mut total = numbers[0];
        for (operator, number) in operator_combination.into_iter().zip(numbers[1..].iter()) {
            total = operator(total, *number);
        }
        if result == total {
            return true
        }
    }
    false
}
