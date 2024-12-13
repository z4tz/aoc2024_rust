use std::iter::Cycle;
use std::slice::Iter;
use crate::Solution;
use std::time::{Duration, Instant};

pub struct Day09 {}

impl Solution for Day09 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = problem_name(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn problem_name(data: &str) -> (String, String) {

    let numbers = data.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
    let mut reverse_files = vec![];
    for (i, file) in numbers.iter().step_by(2).enumerate().rev() {
        for _ in 0..*file {
            reverse_files.push(i as i32);
        }
    }
    let mut files = vec![];
    for (i, file) in numbers.iter().step_by(2).enumerate() {
        for _ in 0..*file {
            files.push(i as i32);
        }
    }
    let max_count = files.len() as i32-1;
    let mut count:i32 = 0;
    let mut numbers_iter = numbers.iter();
    let mut iterators = [files.iter(), reverse_files.iter()];
    let mut indicies:Cycle<Iter<usize>>  = [0, 1].iter().cycle();

    let mut checksum:i64 = 0;
    'outer: loop {
        let iterator = &mut iterators[*indicies.next().unwrap()];

        for _ in 0..*numbers_iter.next().unwrap() {
            let num = *iterator.next().unwrap();
            checksum += (num * count) as i64;
            if count >= max_count{
                break 'outer;
            }
            count += 1;
        }
    }
    (checksum.to_string(), String::new())
}