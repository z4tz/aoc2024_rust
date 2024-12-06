use std::collections::{HashSet};
use crate::Solution;
use std::time::{Duration, Instant};
use num::Complex;

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
        Guard { position: position.clone(), direction: Complex::new(0, -1) }
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

    let mut obstacle_count = 0;
    for position in &visited {
        let tempobstacle = position.clone();
        let mut guard = Guard::new(startposition);
        let mut visited_with_direction: HashSet<(Complex<i32>, Complex<i32>)> = HashSet::new();

        while guard.position.re >= 0 && guard.position.im >= 0 && guard.position.re < dimensions && guard.position.im < dimensions {
            visited_with_direction.insert((guard.position.clone(), guard.direction.clone()));

            guard.step();
            while obstacles.contains(&(guard.position + guard.direction)) || tempobstacle == (guard.position + guard.direction) {
                guard.turn();
            }
            if visited_with_direction.contains(&(guard.position, guard.direction)) {
                obstacle_count += 1;
                break;
            }
        }
    }
    (visited.len() as i32 ,obstacle_count )
}
