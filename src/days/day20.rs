use std::collections::HashSet;
use crate::Solution;
use std::time::{Duration, Instant};
use itertools::enumerate;
use num::Complex;

pub struct Day20 {}

impl Solution for Day20 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = maze_cheating(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn maze_cheating(data: &str) -> (String, String) {
    let mut paths = HashSet::new();
    let mut start:Complex<i32> = Complex::new(0,0);
    let mut end = Complex::new(0,0);

    for (y, line) in enumerate(data.lines()) {
        for (x, char) in enumerate(line.chars()) {
            if char == '.' { paths.insert(Complex::new(x as i32,y as i32)); }
            else if char == 'S' {
                paths.insert(Complex::new(x as i32,y as i32));
                start = Complex::new(x as i32,y as i32);
            }
            else if char == 'E' {
                end = Complex::new(x as i32,y as i32);
                paths.insert(Complex::new(x as i32,y as i32));
            }
        }
    }


    let mut path_vec = vec![start];
    let mut visited = HashSet::new();  // hashset for visited to speed up contains lookup
    visited.insert(start);
    let mut current = start;
    let directions = [Complex::new(1,0), Complex::new(-1, 0), Complex::new(0,1), Complex::new(0, -1)];
    while current != end {
        for direction in  directions {
            let temp_current = current + direction;
            if paths.contains(&temp_current) && !visited.contains(&temp_current) {
                visited.insert(temp_current);
                path_vec.push(temp_current);
                current = temp_current;
                break;  // only one direction can be added
            }
        }
    }
    let part_1 = cheat_path_count(path_vec.clone(), 2, 100);
    let part_2 = cheat_path_count(path_vec, 20, 100);

    (part_1.to_string(), part_2.to_string())
}

fn cheat_path_count(path_vec: Vec<Complex<i32>>, cheat_lenght: i32, time_saving: i32) -> i32{
    let mut count = 0;
    for (i, path) in enumerate(&path_vec[..path_vec.len()-time_saving as usize]) {  //last time_saving elements can be ignored as they cant save that much time
        for (j, goal_path) in enumerate(&path_vec[i..]).skip(time_saving as usize) {  // first time_saving elements can be ignored as they cant save that much time
            let distance = (goal_path - path).l1_norm();
            if distance <= cheat_lenght && j as i32 - distance >= time_saving {
                count += 1;
            }
        }
    }
    count
}