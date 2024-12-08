use std::collections::{HashMap, HashSet};
use crate::Solution;
use std::time::{Duration, Instant};
use itertools::Itertools;
use num::abs;

pub struct Day08 {}

impl Solution for Day08 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = antinode_locations(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

#[derive(Debug)]
struct Antenna {
    x: i32,
    y: i32,
    max_range: i32
}
impl Antenna {
    fn new(x: i32, y: i32, max_range: i32) -> Antenna {
        Antenna { x, y, max_range }
    }
    fn antinodes(&self, other: &Antenna) -> Vec<(i32, i32)> {
        vec![((self.x - other.x) + self.x, (self.y - other.y) + self.y),
             ((other.x - self.x) + other.x, (other.y - self.y) + other.y)]

    }
    fn t_antinodes(&self, other: &Antenna) -> Vec<(i32, i32)> {
        let mut t_antinodes = vec![(self.x, self.y), (other.x, other.y)];
        let mut x_diff = self.x - other.x;
        let mut y_diff = self.y - other.y;
        let gcd = gcd(abs(x_diff) as u64, abs(y_diff) as u64) as i32;
        x_diff /= gcd;
        y_diff /= gcd;
        let mut temp_x = self.x + x_diff;
        let mut temp_y = self.y + y_diff;
        while temp_x >=0 && temp_x < self.max_range && temp_y >=0 && temp_y < self.max_range {
            t_antinodes.push((temp_x, temp_y ));
            temp_x += x_diff;
            temp_y += y_diff;
        }

        let mut temp_x = other.x - x_diff;
        let mut temp_y = other.y - y_diff;
        while temp_x >=0 && temp_x < self.max_range && temp_y >=0 && temp_y < self.max_range {
            t_antinodes.push((temp_x, temp_y));
            temp_x -= x_diff;
            temp_y -= y_diff;
        }
        t_antinodes
    }
}

fn antinode_locations(data: &str) -> (String, String) {
    let mut antennas = HashMap::new();
    let size = data.lines().count() as i32;
    for (y, line) in data.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_insert(vec![]).push(Antenna::new(x as i32, y as i32, size));
            }
        }
    }
    let mut valid_antinodes = HashSet::new();
    let mut valid_t_antinodes = HashSet::new();
    for antenna_vec in antennas.values() {
        for positions in antenna_vec.iter().combinations(2) {
            //part 1
            for (x, y) in positions[0].antinodes(&positions[1]) {
                if x >= 0 && x < size && y >= 0 && y < size {
                    valid_antinodes.insert((x, y));
                }
            }
            //part 2
            for (x, y) in positions[0].t_antinodes(&positions[1]) {
                valid_t_antinodes.insert((x, y));
            }
        }
    }
    (valid_antinodes.len().to_string(), valid_t_antinodes.len().to_string())
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}