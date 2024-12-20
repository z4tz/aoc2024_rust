
use crate::Solution;
use std::time::{Duration, Instant};
use itertools::Itertools;
use num::pow;

pub struct Day17 {}

impl Solution for Day17 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = problem_name(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn problem_name(data: &str) -> (String, String) {
    let mut parts = data.split("\r\n\r\n");
    let register_string = parts.next().unwrap();
    let temp = register_string.lines().map(|s|s.split(" ").last().unwrap().to_string().parse::<i32>().unwrap()).collect_vec();
    let mut registers = [temp[0], temp[1], temp[2]];

    let program_string = parts.next().unwrap().split(" ").skip(1).next().unwrap();
    let program = program_string.split(",").map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();


    let mut pointer = 0;
    let mut output = vec![];

    loop {
        if pointer >= program.len() {
            break;
        }
        let opcode = program[pointer];
        let operand = program[pointer+1];
        match opcode {
            0 => {registers[0] = dv(registers, operand); pointer += 2;},
            1 => {registers[1] = registers[1]^operand; pointer += 2;},
            2 => {registers[1] = bst(registers, operand); pointer += 2;},
            3 => {if registers[0]>0 {pointer = operand as usize} else { pointer += 2}},
            4 => {registers[1] = registers[1]^registers[2]; pointer += 2;},
            5 => {output.push(combo_operand(registers, operand)%8); pointer += 2;},
            6 => {registers[1] = dv(registers, operand); pointer += 2;}
            7 => {registers[2] = dv(registers, operand); pointer += 2;}
            _ => println!("Invalid opcode: {}", opcode)
        }
    }
    (output.into_iter().join(","), String::new())
}

fn dv(registers: [i32; 3], operand: i32) -> i32 {
    let numerator = registers[0];
    let denominator = pow(2, combo_operand(registers, operand) as usize);
    numerator / denominator
}

fn bst(registers: [i32; 3], operand: i32) -> i32 {
    combo_operand(registers, operand)%8
}

fn combo_operand(registers: [i32; 3], operand: i32) -> i32 {
    match operand {
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        7 => panic!("combo operand 7 used"),
        _ => operand
    }
}