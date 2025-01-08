use std::collections::HashMap;
use crate::Solution;
use std::time::{Duration, Instant};
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

pub struct Day19 {}

impl Solution for Day19 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let result1 = possible_patterns(data);
        let result2 = pattern_permutations(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn possible_patterns(data: &str) -> String {
    let mut lines = data.lines();
    let pattern = format!("^({})+$",lines.next().unwrap().replace(", ", "|"));
    let re = Regex::new(pattern.as_str()).unwrap();
    lines.next().unwrap();  // skip empty row
    let total = lines.fold(0, |acc, line| if re.is_match(line) { acc + 1 } else { acc });

    total.to_string()
}

fn pattern_permutations(data: &str) -> String {
    let mut lines = data.lines();
    let patterns = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    lines.next().unwrap();
    let total: i64 = lines.collect_vec().into_par_iter()  // parallel computation gains a factor ~10
        .map(|line| match_string(line, &patterns, &mut HashMap::new()))
        .sum();

    total.to_string()
}

fn match_string<'a>(str: &'a str, patterns: &Vec<&str>, memoized_values: &mut HashMap<&'a str, i64>) -> i64 {
    let mut total = 0;
    for pattern in patterns {
        if str.starts_with(pattern) {
            let new_string = &str[pattern.len()..];
            if new_string.is_empty() {
                return 1;
            }
            if memoized_values.contains_key(new_string) {
                total += memoized_values.get(new_string).unwrap();
            }
            else {
                let result = match_string(new_string, patterns, memoized_values);
                memoized_values.insert(new_string, result);
                total += result;
            }
        }
    }
    total
}

