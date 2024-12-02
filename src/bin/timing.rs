use std::time::Duration;
use aoc2024_rust::days;
use aoc2024_rust::{aoc_reader, Solution};

fn main() {
    let day = 2;
    let data = aoc_reader(&day);
    let problem = get_problem(&day).unwrap();
    let (result1, result2, time) = problem.timed_solution(&data);
    println!(
        "Solving day {day}\n\
         Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );
    let repeat_count = (1.0 /time.as_secs_f64()) as i32;
    let mut total_time = Duration::new(0, 0);
    for i in 0..repeat_count {
        let (_, _, time) = problem.timed_solution(&data);
        total_time += time;
    }
    let avg_time = total_time / repeat_count as u32;
    println!("avg_time {:?} over {} attempts", avg_time, repeat_count);
}

fn get_problem(day:&i8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(days::day01::Day01{})),
        2 => Some(Box::new(days::day02::Day02{})),
        _ => Some(Box::new(days::day01::Day01{}))
    }
}