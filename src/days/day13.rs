use crate::Solution;
use std::time::{Duration, Instant};
use itertools::Itertools;
use regex::Regex;

pub struct Day13 {}

impl Solution for Day13 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = token_optimizer(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn token_optimizer(data: &str) -> (String, String) {
    let number_regex = Regex::new(r"(\d+)").unwrap();
    let mut tokens1 = 0.0;
    let mut tokens2 = 0.0;
    for matches in number_regex.find_iter(data).collect_vec().chunks_exact(6) {
        let (ax, ay, bx, by, mut px, mut py) = matches.iter().map( |x| x.as_str().parse::<f64>().unwrap()).collect_tuple().unwrap();
        let (a_count, b_count) = two_equation_substitution(ax, ay, bx, by, px, py);
        if a_count.fract() + b_count.fract() == 0.0 {  // if integer solutions could be found,
            tokens1 += a_count * 3.0 + b_count;
        }
        //part 2
        px += 10_000_000_000_000.0;
        py += 10_000_000_000_000.0;
        let (a_count, b_count) = two_equation_substitution(ax, ay, bx, by, px, py);
        if a_count.fract() + b_count.fract() == 0.0 {  // if integer solutions could be found,
            tokens2 += a_count * 3.0 + b_count;
        }
    }


    (tokens1.to_string(), tokens2.to_string())
}

fn two_equation_substitution(ax: f64, ay: f64, bx: f64, by: f64, prize_x: f64, prize_y: f64) -> (f64, f64) {
    let y_result = (prize_x * ay - prize_y * ax) / (bx * ay - ax * by);
    let x_result = (prize_x - bx * y_result) / ax;
    (x_result, y_result)
}

