use std::collections::HashSet;
use ndarray::Array;
use crate::Solution;
use std::time::{Duration, Instant};
use itertools::Itertools;

pub struct Day09 {}

impl Solution for Day09 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = problem_name(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn problem_name(data: &str) -> (String, String) {
    let dimension = data.lines().count();
    let int_vector = data.replace("\r\n","").chars().flat_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
    let arr = Array::from_shape_vec((dimension,dimension),int_vector).unwrap();
    let mut score = 0;
    let starting_positions = arr.iter().positions(|&e|e == 0).map(|i|index_to_pos(i, dimension)).collect::<Vec<_>>();
    for start_position in starting_positions {
        let mut current_positions = HashSet::from([start_position]);
        for i in 1..10 {
            let mut next_positions = HashSet::new();
            println!("Checking the following positions {current_positions:?}");
            for position in &current_positions {
                for neighbor in get_neighbours(*position) {
                    //println!("Checking position {neighbor:?} for {i:?}");
                    if arr.get(neighbor).unwrap_or(&0) == &i {
                        println!("{neighbor:?} is a match for {i}");
                        next_positions.insert(neighbor);
                    }
                }
                println!("");
            }
            current_positions.clear();
            current_positions.extend(next_positions.clone());




        }
        score += current_positions.len();
    }



    (score.to_string(), String::new())
}
type Position = (usize, usize);
fn get_neighbours(p: Position) -> impl Iterator<Item = Position> {
    let m_range = if p.0 > 0 { p.0 - 1..p.0 + 2 } else { 0..2 };
    let n_range = if p.1 > 0 { p.1 - 1..p.1 + 2 } else { 0..2 };
    m_range
        .flat_map(move |m| n_range.clone().map(move |n| (m, n)))
        .filter(move |&q| p != q)
}

fn index_to_pos(i: usize, dimension: usize) -> Position {
    Position::from((i/dimension,i%dimension))
}