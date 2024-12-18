use crate::Solution;
use std::time::{Duration, Instant};
use itertools::Itertools;
use ndarray::prelude::*;
use regex::Regex;

pub struct Day14 {}

impl Solution for Day14 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = robot_movement(data);
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
    fn after_steps(&self, steps: i32, dimensions: (i32, i32)) -> (i32, i32) {
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

fn robot_movement(data: &str) -> (String, String) {
    let re = Regex::new(r"p=([-\d]+),([-\d]+) v=([-\d]+),([-\d]+)").unwrap();
    let dims = (101,103);
    let time = 30;
    let robots = re.captures_iter(data)
        .map(|a|a.iter().skip(1)
            .map(|a|a.unwrap().as_str().parse::<i32>().unwrap()).collect_vec())
        .map(|a| Robot::new(a[0], a[1], a[2], a[3])).collect_vec();

    let positions = &robots.iter()
        .map(|robot| robot.after_steps(time, dims))
        .collect_vec();
    let mut quarters = [0;4];
    for position in positions {
        if position.0 < dims.0/2  && position.1 < dims.1/2 {
            quarters[0] += 1;
        }
        else if position.0 > dims.0/2 && position.1 < dims.1/2 {
            quarters[1] += 1;
        }
        else if position.0 < dims.0/2 && position.1 > dims.1/2 {
            quarters[2] += 1;
        }
        else if position.0 > dims.0/2  && position.1 > dims.1/2 {
            quarters[3] += 1;
        }
    }

    let mut image = Array::from_elem((dims.1 as usize, dims.0 as usize), ' ');

    let positions = &robots.iter()
        .map(|robot| robot.after_steps(33 + 76*103, dims))  // found by manualy noticing pattern by step 33 + 103n
        .collect_vec();
    for position in positions {
        image[(position.1 as usize, position.0 as usize)] = '#';
    }
    //println!("{:#}", image);

    (quarters.iter().product::<i32>().to_string(), "7861".to_string())
}
