
use crate::Solution;
use std::time::{Duration, Instant};
use itertools::{Itertools};
use regex::Regex;

pub struct Day14 {}

impl Solution for Day14 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = problem_name(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32)
}
impl Robot {
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Robot {
        Robot { position:(x,y), velocity:(vx,vy) }
    }
    fn step(&self, steps: i32, dimensions: (i32, i32)) -> (i32, i32) {
        let mut position = ((self.position.0 + self.velocity.0 * steps)%dimensions.0,
        (self.position.1 + self.velocity.1 * steps)%dimensions.1);
        if position.0 < 0 {
            position.0 += dimensions.0;
        }
        if position.1 < 0 {
            position.1 += dimensions.1;
        }
        position
    }
}


fn problem_name(data: &str) -> (String, String) {
    let re = Regex::new(r"p=([-\d]+),([-\d]+) v=([-\d]+),([-\d]+)").unwrap();
    let dims = (11,7);
    let time = 5;
    let robots = re.captures_iter(data)
        .map(|a|a.iter().skip(1)
            .map(|a|a.unwrap().as_str().parse::<i32>().unwrap()).collect_vec())
        .map(|a| Robot::new(a[0], a[1], a[2], a[3])).collect_vec();

    let positions = robots.iter()
        .map(|robot| robot.step(time, dims))
        .collect_vec();
    println!("{:?}", positions);



    // println!("{:?}", robots);



    (String::new(), String::new())
}

