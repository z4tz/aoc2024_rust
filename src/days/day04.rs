use crate::Solution;
use ndarray::{Array, Axis};
use std::time::{Duration, Instant};

pub struct Day04 {}

impl Solution for Day04 {
    fn timed_solution(&self, data: &str) -> (i32, i32, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = find_xmas(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn find_xmas(data: &str) -> (i32, i32) {
    let dimension = data.lines().count();
    let char_vector :Vec<char> = data.replace("\r\n","").chars().collect();
    let arr = Array::from_shape_vec((dimension,dimension),char_vector).unwrap();

    //part 1
    let mut xmas_count = 0;
    //columns
    for window in arr.windows((4,1)).into_iter() {
        xmas_count += match_xmas(&window.iter().collect::<String>());
    }
    //rows
    for window in arr.windows((1,4)).into_iter() {
        xmas_count += match_xmas(&window.iter().collect::<String>());
    }
    //diagonals
    for mut window in arr.windows((4,4)).into_iter() {
        xmas_count += match_xmas(&window.diag().iter().collect::<String>());
        window.invert_axis(Axis(0));
        xmas_count += match_xmas(&window.diag().iter().collect::<String>());
    }

    //part 2
    let mut mas_count = 0;
    for mut window in arr.windows((3,3)).into_iter() {
        let s1 = window.diag().iter().collect::<String>();
        window.invert_axis(Axis(0));
        let s2 = window.diag().iter().collect::<String>();
        if match_mas(&s1) && match_mas(&s2) {
            mas_count += 1;
        }
    }
    (xmas_count,mas_count)
}

fn match_xmas(s: &str) -> i32 {
    match s {
        "XMAS" => 1,
        "SAMX" => 1,
        _ => 0
    }
}

fn match_mas(s: &str) -> bool {
    match s {
        "MAS" => true,
        "SAM" => true,
        _ => false
    }
}
