use aoc2024_rust::aoc_reader;
use std::time::Instant;
use num::abs;

fn main() {
    let data = aoc_reader(1);
    let start = Instant::now(); // skip file IO in timing
    let (result1, result2) = list_comparison(&data);
    let duration = start.elapsed();
    println!(
        "Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );
    println!("Execution took {:?}", duration)
}

fn list_comparison(data: &str) -> (i32, i32) {
    let mut first: Vec<i32> = vec!();
    let mut second: Vec<i32> = vec!();

    for line in data.lines() {
        let parts: Vec<i32> = line.split("   ").map(|x| x.parse().unwrap()).collect();
        first.push(parts[0]);
        second.push(parts[1]);
    }
    
    first.sort();
    second.sort();

    let diff: i32 = first.iter().zip(second.iter()).map(|(a, b)| abs(a - b)).sum();

    //Part 2
    let mut similarity_score = 0;
    for number in first {
        similarity_score += second.iter().filter(|&x| *x == number).count() as i32 * number;
    }
    
    (diff, similarity_score)
}
