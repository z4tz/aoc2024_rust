use std::env;
use itertools::Itertools;
use aoc2024_rust::{aoc_reader, get_solution};

fn main() {
    let args = env::args().collect_vec();
    let day = if args.len() > 1 {args[1].parse::<i8>().expect("Invalid integer(i8) input")}
    else {20};

    let solution = get_solution(&day).expect("Unable to get solution");
    let (result1, result2, _) = solution.timed_solution(&aoc_reader(&day));
    println!(
        "Solving day {day}\n\
         Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );

}