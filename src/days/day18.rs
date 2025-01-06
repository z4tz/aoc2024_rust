use std::collections::HashSet;
use std::ops::Index;
use crate::Solution;
use std::time::{Duration, Instant};
use itertools::Itertools;
use num::Complex;

pub struct Day18 {}

impl Solution for Day18 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = corruption_pathfinding(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn corruption_pathfinding(data: &str) -> (String, String) {
    let dimensions = (71,71);
    let mut corrupted = vec![];
    for line in data.lines() {
        //println!("{}", line);
        let coordinate = line.split(",").map( | p |p.parse::<i32>().unwrap()).collect_vec();
        corrupted.push(Complex::new(coordinate[0], coordinate[1]));
    }

    //part 1
    let counter = count_steps(&dimensions, &mut HashSet::from_iter(corrupted[..1024].iter()));

    //part 2
    let mut upper = corrupted.len();
    let mut lower = 1024;
    while upper - lower > 1 {
        let midpoint = lower + (upper - lower) / 2;
        if count_steps(&dimensions, &mut HashSet::from_iter(corrupted[..midpoint].iter())) > 0 {
            lower = midpoint;
        }
        else {
            upper = midpoint;
        }
    }
    let blocking_corruption = corrupted.index(upper-1);

    (counter.to_string(), format!("{},{}", blocking_corruption.re, blocking_corruption.im))
}

fn count_steps(dimensions: &(i32, i32), corrupted: &mut HashSet<&Complex<i32>>) -> i32 {
    let end = Complex::new(dimensions.0 - 1, dimensions.1 - 1);

    let mut visited = HashSet::new();
    let mut current_visits = HashSet::from([Complex::new(0, 0)]);
    let mut next_visit = HashSet::new();
    let mut counter = 0;
    let mut found_exit = false;
    'outer: while current_visits.len() != 0 {
        for location in current_visits.iter() {
            if location == &end {
                found_exit = true;
                break 'outer;
            }

            for neighbor in valid_neighbors(location, &dimensions) {
                if !visited.contains(&neighbor) && !corrupted.contains(&neighbor) {
                    next_visit.insert(neighbor.clone());
                    visited.insert(neighbor);
                }
            }
        }
        current_visits = next_visit;
        next_visit = HashSet::new();
        counter += 1;
    }
    match found_exit {
        true => counter,
        false => 0
    }
}

fn valid_neighbors(coordinate1: &Complex<i32>, dimensions: &(i32, i32)) -> Vec<Complex<i32>> {
    let mut neighbors = vec![];
    for direction in [Complex::new(1, 0), Complex::new(-1, 0), Complex::new(0, 1), Complex::new(0, -1)] {
        let neighbor = coordinate1 + direction;
        if neighbor.re >= 0 && neighbor.im >= 0  && neighbor.re < dimensions.0  && neighbor.im < dimensions.1 {
            neighbors.push(neighbor);
        }
    }
    neighbors
}