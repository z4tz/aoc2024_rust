use crate::Solution;
use std::time::{Duration, Instant};

pub struct Day05 {}

impl Solution for Day05 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = page_ordering(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn page_ordering(data: &str) -> (String, String) {
    let mut rules :Vec<Vec<i32>> = vec![];
    let mut manuals :Vec<Vec<i32>> = vec![];

    let mut input_break = false;
    for line in data.lines() {
        if !input_break {
            if line.is_empty() {
                input_break = true;
                continue;
            }
            rules.push(line.split("|").map( | p |p.parse::<i32>().unwrap()).collect());
        }
        else {
            manuals.push(line.split(",").map( | p |p.parse::<i32>().unwrap()).collect());
        }
    }

    let mut middle_page_sum = 0;
    let mut sorted_middle_page_sum = 0;
    for mut pages in manuals {
        let has_changed = sort_pages(&mut pages, &rules);
        let middle_index = pages.len() / 2;
        match has_changed {
            false => middle_page_sum += pages[middle_index],  // part 1
            true => sorted_middle_page_sum += pages[middle_index] // part 2
        }
    }
    (middle_page_sum.to_string(),sorted_middle_page_sum.to_string())
}

fn sort_pages(pages: &mut Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    let mut has_changed = false;
    let mut sorted = false;
    while !sorted {
        sorted = true;
        for rule in rules {
            if pages.contains(&rule[0]) && pages.contains(&rule[1]) {
                let index_a = pages.iter().position(|&page| page == rule[0]).unwrap();
                let index_b = pages.iter().position(|&page| page == rule[1]).unwrap();
                if index_a > index_b {
                    pages.swap(index_a, index_b);
                    sorted = false;
                    has_changed = true;
                }
            }
        }
    }
    has_changed
}