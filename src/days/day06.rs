use std::collections::{HashSet};
use crate::Solution;
use std::time::{Duration, Instant};
use num::Complex;
use rayon::prelude::*;

pub struct Day06 {}

impl Solution for Day06 {
    fn timed_solution(&self, data: &str) -> (i32, i32, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = guard_patterns(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

#[derive(Debug)]
struct Guard {
    position: Complex<i32>,
    direction: Complex<i32>
}
impl Guard {
    fn new(position: Complex<i32>) -> Guard {
        Guard { position, direction: Complex::new(0, -1) }
    }
    fn step(&mut self) {
        self.position += self.direction
    }
    fn turn(&mut self) {
        self.direction *= Complex::i()
    }
}

fn guard_patterns(data: &str) -> (i32, i32) {
    let mut visited:HashSet<Complex<i32>> = HashSet::new();
    let mut obstacles:HashSet<Complex<i32>> = HashSet::new();
    let mut startposition = Complex::new(0, 0);
    let dimensions = data.lines().count() as i32;
    for (y, line) in data.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                obstacles.insert(Complex::new(x as i32, y as i32));
            }
            if char == '^' {
                startposition = Complex::new(x as i32, y as i32);
            }
        }
    }
    let mut guard = Guard::new(startposition);


    while guard.position.re >= 0 && guard.position.im >= 0 &&  guard.position.re < dimensions && guard.position.im < dimensions {
        visited.insert(guard.position.clone());

        guard.step();
        while obstacles.contains(&(guard.position + guard.direction)) {
            guard.turn();
        }
    }
    let visited_count = visited.len() as i32;

    let obstacle_count = visited.into_par_iter()
        .map(|x| {
            let mut obstacles_clone = obstacles.clone();
            detect_loop(&mut obstacles_clone, startposition, dimensions, x)
        })
        .sum();

        (visited_count,obstacle_count)
}

fn detect_loop(obstacles: &mut HashSet<Complex<i32>>, startposition: Complex<i32>, dimensions: i32, tempobstacle: Complex<i32>) -> i32{
    let mut guard = Guard::new(startposition);
    let mut visited_with_direction: HashSet<(Complex<i32>, Complex<i32>)> = HashSet::new();

    while guard.position.re >= 0 && guard.position.im >= 0 && guard.position.re < dimensions && guard.position.im < dimensions {
        visited_with_direction.insert((guard.position.clone(), guard.direction.clone()));

        guard.step();
        while obstacles.contains(&(guard.position + guard.direction)) || tempobstacle == (guard.position + guard.direction) {
            guard.turn();
        }
        if visited_with_direction.contains(&(guard.position, guard.direction)) {
            return 1
        }
    }
    0
}
