use std::time::{Duration, Instant};
use crate::Solution;

pub struct Day02 {}

impl Solution for Day02 {
    fn timed_solution(&self, data: &str) -> (i32, i32, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = safe_report_count(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn safe_report_count(data: &str) -> (i32, i32) {
    let mut safe_reports_1 = 0;
    let mut safe_reports_2 = 0;
    for line in data.lines() {
        let mut numbers = line.split(" ").map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if numbers[0] > numbers[numbers.len() - 1] {  //breaks if first or last is bad enough to change ordering
            numbers.reverse();
        }
        if is_safe(numbers.clone()) {
            safe_reports_1 += 1;
            safe_reports_2 += 1;
        }
        else {
            for i in 0..numbers.len() {
                let mut tempnumbers = numbers.clone();
                tempnumbers.remove(i);
                if is_safe(tempnumbers) {
                    safe_reports_2 += 1;
                    break;
                }
            }
        }
    }
    (safe_reports_1, safe_reports_2)
}

fn is_safe(numbers: Vec<i32>) -> bool {
    for window in numbers.windows(2) {
        let diff = window[1] - window[0];
        if diff < 1 || diff > 3 {
            return false
        }
    }
    return true
}