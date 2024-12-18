use std::collections::{BTreeSet};
use crate::Solution;
use std::time::{Duration, Instant};
use ndarray::Array;


pub struct Day12 {}

impl Solution for Day12 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = problem_name(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

type Position = (usize, usize);

fn problem_name(data: &str) -> (String, String) {
    let dimension = data.lines().count();
    let char_vector :Vec<char> = data.replace("\r\n","").chars().collect();
    let arr = Array::from_shape_vec((dimension,dimension),char_vector).unwrap();
    let mut visited = BTreeSet::<Position>::new();
    let mut price = 0;
    for y in 0..dimension {
        for x in 0..dimension {
            let mut current_position = Position::from([y,x]);
            if !visited.contains(&current_position) {
                let mut to_visit = BTreeSet::<Position>::new();
                to_visit.insert(current_position);
                let field_type = arr.get(current_position).unwrap();
                let mut area = 0;
                let mut perimeter = 0;
                while to_visit.len() > 0 {
                    //println!("{to_visit:?}");
                    current_position = to_visit.pop_first().unwrap();
                    visited.insert(current_position);
                    area += 1;
                    let mut possible_perimeter = 4;
                    for neighbor in get_neighbours(current_position) {
                        if arr.get(neighbor).unwrap_or(&' ') == field_type {
                            possible_perimeter -= 1;
                            if !visited.contains(&neighbor) {
                                to_visit.insert(neighbor);
                            }
                        }
                    }
                    perimeter += possible_perimeter;

                }
                println!("{area} {perimeter} {field_type}");
                price += area * perimeter;
            }



        }
    }

    (price.to_string(), String::new())
}

fn get_neighbours(p: Position) -> impl Iterator<Item =Position> {
    [Position::from([p.0-1,p.1]), Position::from([p.0+1,p.1]), Position::from([p.0,p.1-1]), Position::from([p.0,p.1+1])].into_iter()
}