use std::time::Duration;
use aoc2024_rust::{
    aoc_reader,
    days,
    Solution
};

fn main() {
    let day = 5;
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
        5 => Some(Box::new(days::day05::Day05{})),
        6 => Some(Box::new(days::day06::Day06{})),
        7 => Some(Box::new(days::day07::Day07{})),
        8 => Some(Box::new(days::day08::Day08{})),
        9 => Some(Box::new(days::day09::Day09{})),
        10 => Some(Box::new(days::day10::Day10{})),
        11 => Some(Box::new(days::day11::Day11{})),
        12 => Some(Box::new(days::day12::Day12{})),
        13 => Some(Box::new(days::day13::Day13{})),
        14 => Some(Box::new(days::day14::Day14{})),
        15 => Some(Box::new(days::day15::Day15{})),
        16 => Some(Box::new(days::day16::Day16{})),
        17 => Some(Box::new(days::day17::Day17{})),
        18 => Some(Box::new(days::day18::Day18{})),
        19 => Some(Box::new(days::day19::Day19{})),
        20 => Some(Box::new(days::day20::Day20{})),
        21 => Some(Box::new(days::day21::Day21{})),
        22 => Some(Box::new(days::day22::Day22{})),
        23 => Some(Box::new(days::day23::Day23{})),
        24 => Some(Box::new(days::day24::Day24{})),
        25 => Some(Box::new(days::day25::Day25{})),
        _ => None
    }
}
