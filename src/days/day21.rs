use std::collections::{HashMap, HashSet};
use crate::Solution;
use std::time::{Duration, Instant};
use itertools::Itertools;
use num::Complex;

pub struct Day21 {}

impl Solution for Day21 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = problem_name(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn problem_name(data: &str) -> (String, String) {
    let mut robots = vec![Robot::new(true), Robot::new(false), Robot::new(false)];
    let mut complexity_sum = 0;
    for line in data.lines() {
        let number = 0;//line[0..3].parse::<usize>().unwrap();
        let mut instructions = vec![line.to_owned()];
        for robot in &mut robots {
            let mut new_instructions = vec![];
            for instruction in instructions {
                new_instructions.push(robot.multi_instruction(&instruction));
                robot.reset();
            }
            let temp_instructions = new_instructions.into_iter().flatten().collect_vec();
            println!("{:?}", temp_instructions.len());
            let unique:HashSet<String> = HashSet::from_iter(temp_instructions.iter().cloned());
            instructions = unique.into_iter().collect_vec();
            println!("{:?}", instructions.len());
        }
        let instruction = instructions.iter().min_by(|x, y| x.len().cmp(&y.len())).unwrap();

        println!("{} {} {}", instruction.len(), number, instruction.len() * number);
        complexity_sum += instruction.len() * number;
    }
    (complexity_sum.to_string(), String::new())
}

struct Robot {
    current: Complex<i32>,
    board: HashMap<char, Complex<i32>>,
    large_board: bool
}
impl Robot {

    fn get_instructions(&mut self, button: char) -> Vec<String> {
        let mut instruction = String::new();
        let movement = self.board.get(&button).unwrap() - self.current;
        //println!("{} {} {movement}",self.current, self.board.get(&button).unwrap());

        if movement.im < 0 {instruction.push_str(&*"^".repeat(movement.im.abs() as usize));}
        else               {instruction.push_str(&*"v".repeat(movement.im.abs() as usize));}


        if movement.re > 0 {instruction.push_str(&*">".repeat(movement.re as usize));}
        else               {instruction.push_str(&*"<".repeat(movement.re.abs() as usize));}
        let instructions:Vec<Vec<char>> = instruction.chars().permutations(instruction.len()).collect();
        let mut test: Vec<String> = instructions.iter().map(|a| a.iter().collect()).collect();
        for instruction in &mut test {
            instruction.push('A');
        }

        self.current = *self.board.get(&button).unwrap();

        test
    }

    fn multi_instruction(&mut self, buttons: &str) -> Vec<String> {
        let mut part_instructions = vec![];
        for character in buttons.chars() {
            part_instructions.push(self.get_instructions(character));
        }
        let mut combined_instructions = part_instructions.first().unwrap().clone();
        for part_instruction in part_instructions[1..].iter() {
            let mut new_instructions = vec![];
            for instruction1 in combined_instructions {
                for instruction2 in part_instruction {
                    new_instructions.push(format!("{instruction1}{instruction2}"));
                }
            }
            combined_instructions = new_instructions;
        }
        combined_instructions
    }

    fn reset(&mut self) {
        self.current = *self.board.get(&'A').unwrap();
    }

    fn new(large_board: bool) -> Robot {
        if large_board {
            Robot { current: Complex::new(2, 3), board: Robot::large_board(), large_board }
        } else {
            Robot { current: Complex::new(2, 0), board: Robot::small_board(), large_board }
        }
    }
    fn large_board() -> HashMap<char, Complex<i32>> {
        let mut board = HashMap::new();
        board.insert('7', Complex::new(0,0));
        board.insert('8', Complex::new(1,0));
        board.insert('9', Complex::new(2,0));
        board.insert('4', Complex::new(0,1));
        board.insert('5', Complex::new(1,1));
        board.insert('6', Complex::new(2,1));
        board.insert('1', Complex::new(0,2));
        board.insert('2', Complex::new(1,2));
        board.insert('3', Complex::new(2,2));
        board.insert('0', Complex::new(1,3));
        board.insert('A', Complex::new(2,3));

        board
    }
    fn small_board() -> HashMap<char, Complex<i32>> {
        let mut board = HashMap::new();
        board.insert('^', Complex::new(1,0));
        board.insert('A', Complex::new(2,0));
        board.insert('<', Complex::new(0,1));
        board.insert('v', Complex::new(1,1));
        board.insert('>', Complex::new(2,1));

        board
    }
}
