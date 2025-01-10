pub mod days;

use std::fs;
use std::time::Duration;

pub fn aoc_reader(day: &i8) -> String {
    let filename = format!("inputs/day{day}.txt");
    let content = fs::read_to_string(&filename);
    if content.is_err() {
        println!("Couldn't read file {}", filename);
        return "".to_string()
    }
    else { 
        return content.unwrap()
    }
}


pub trait Solution {
    fn timed_solution(&self, data: &str) -> (String, String, Duration);
}

pub fn get_solution(day:&i8) -> Option<Box<dyn Solution>> {
    match day {
        1 => Some(Box::new(days::day01::Day01{})),
        2 => Some(Box::new(days::day02::Day02{})),
        3 => Some(Box::new(days::day03::Day03{})),
        4 => Some(Box::new(days::day04::Day04{})),
        5 => Some(Box::new(days::day05::Day05{})),
        6 => Some(Box::new(days::day06::Day06{})),
        7 => Some(Box::new(days::day07::Day07{})),
        8 => Some(Box::new(days::day08::Day08{})),
        9 => Some(Box::new(days::day09::Day09{})),
        10 => Some(Box::new(days::day10::Day10{})),
        11 => Some(Box::new(days::day11::Day11{})),
        12 => Some(Box::new(days::day12::Day12{})),
        13 => Some(Box::new(days::day13::Day13{})),
        14 => Some(Box::new(days::day14::Day14{})),
        15 => Some(Box::new(days::day15::Day15{})),
        16 => Some(Box::new(days::day16::Day16{})),
        17 => Some(Box::new(days::day17::Day17{})),
        18 => Some(Box::new(days::day18::Day18{})),
        19 => Some(Box::new(days::day19::Day19{})),
        20 => Some(Box::new(days::day20::Day20{})),
        21 => Some(Box::new(days::day21::Day21{})),
        22 => Some(Box::new(days::day22::Day22{})),
        23 => Some(Box::new(days::day23::Day23{})),
        24 => Some(Box::new(days::day24::Day24{})),
        25 => Some(Box::new(days::day25::Day25{})),
        _ => None
    }
}

#[macro_export]
macro_rules! pause {
    () => {
        println!(
            "[{}:{}] Pausing! Press enter to continue...",
            file!(),
            line!()
        );

        let mut buffer = String::new();

        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");
    };
}