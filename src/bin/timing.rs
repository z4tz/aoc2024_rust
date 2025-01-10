use std::time::Duration;
use aoc2024_rust::{
    aoc_reader,
    get_solution,
};

fn main() {
    let day = 21;
    let runtime = 1;
    let data = aoc_reader(&day);
    let solution = get_solution(&day).expect("Unable to get solution");
    let (result1, result2, mut time) = solution.timed_solution(&data);
    println!(
        "Solving day {day}\n\
         Part 1 result: {result1}\n\
         Part 2 result: {result2}"
    );

    let mut run_count = 1;
    while time < Duration::from_secs(runtime) {
        let (_, _, new_time) = solution.timed_solution(&data);
        time += new_time;
        run_count += 1;
    }

    let avg_time = time / (run_count);
    println!("Average time {:?} over {} attempts", avg_time, run_count);
}
