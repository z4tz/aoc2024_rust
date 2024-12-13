use std::collections::HashSet;
use ndarray::{Array, ArrayBase, Dim, Ix, OwnedRepr};
use crate::Solution;
use std::time::{Duration, Instant};
use itertools::Itertools;

pub struct Day10 {}

impl Solution for Day10 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = trail_scores(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

type Position = (usize, usize);

fn trail_scores(data: &str) -> (String, String) {
    let dimension = data.lines().count();
    let int_vector = data.replace("\r\n","").chars().flat_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
    let arr = Array::from_shape_vec((dimension,dimension),int_vector).unwrap();
    let mut score1 = 0;
    let mut score2 = 0;
    let starting_positions = arr.iter().positions(|&e|e == 0).map(|i|Position::from((i/dimension,i%dimension))).collect::<Vec<_>>();
    for start_position in starting_positions {
        let trailtails = step(start_position, 1, &arr);
        score2 += trailtails.len();
        score1 += (HashSet::from_iter(trailtails.iter().cloned()) as HashSet<Position>).len();

    }

    (score1.to_string(), score2.to_string())
}

fn step(position: Position, look_foor: u32, arr:  &ArrayBase<OwnedRepr<u32>, Dim<[Ix; 2]>>) ->Vec<Position> {
    if look_foor == 10 {
        return vec![position];
    }
    let mut trailtails = vec![];

    for neighbor in get_neighbours(position) {
        if arr.get(neighbor).unwrap_or(&0) == &look_foor {
            trailtails.extend(step(neighbor, look_foor+1, arr));
        }
    }
    trailtails
}

fn get_neighbours(p: Position) -> impl Iterator<Item = Position> {
    [Position::from([p.0-1,p.1]), Position::from([p.0+1,p.1]), Position::from([p.0,p.1-1]), Position::from([p.0,p.1+1])].into_iter()
}