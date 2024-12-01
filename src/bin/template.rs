use aoc2024_rust::aoc_reader;
use std::time::Instant;

fn main() {
    let data = aoc_reader(0);
    let start = Instant::now(); // skip file IO in timing
    let result1 = part1(&data);
    let result2 = part2(&data);
    let duration = start.elapsed();
    println!(
        "Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );
    println!("Execution took {:?}", duration)
}

fn part1(data: &str) -> i32 {
    1
}

fn part2(data: &str) -> i32 {
    2
}
