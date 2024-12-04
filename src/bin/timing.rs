use std::time::Duration;
use aoc2024_rust::{
    aoc_reader,
    days,
    Solution
};

fn main() {
    let day = 4;
    let runtime = 1f64;
    let data = aoc_reader(&day);
    let solution = get_solution(&day).expect("Unable to get solution");
    let (result1, result2, time) = solution.timed_solution(&data);
    println!(
        "Solving day {day}\n\
         Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );
    let repeat_count = (runtime /time.as_secs_f64()) as i64;
    let mut total_time = Duration::new(0, 0);
    total_time += time;
    for _ in 0..repeat_count {
        let (_, _, time) = solution.timed_solution(&data);
        total_time += time;
    }
    let avg_time = total_time / (repeat_count+1) as u32;
    println!("Average time {:?} over {} attempts", avg_time, repeat_count + 1);
}

fn get_solution(day:&i8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(days::day01::Day01{})),
        2 => Some(Box::new(days::day02::Day02{})),
        3 => Some(Box::new(days::day03::Day03{})),
        4 => Some(Box::new(days::day04::Day04{})),
        _ => None
    }
}
