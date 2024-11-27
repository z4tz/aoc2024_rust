use aoc2024_rust::aoc_reader;
use std::time::Instant;

fn main() {
    let content = aoc_reader(0);
    let start = Instant::now(); // skip file IO in timing
    let result1 = part1(&content);
    let result2 = part2(&content);
    let duration = start.elapsed();
    println!(
        "Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );
    println!("Execution took {:?}", duration)
}

fn part1(content: &str) -> i32 {
    1
}

fn part2(content: &str) -> i32 {
    2
}
