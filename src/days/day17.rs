
use crate::Solution;
use std::time::{Duration, Instant};
use itertools::Itertools;
use num::pow;

pub struct Day17 {}

impl Solution for Day17 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = octal_computer(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn octal_computer(data: &str) -> (String, String) {
    let mut parts = data.split("\r\n\r\n");
    let register_string = parts.next().unwrap();
    let temp = register_string.lines().map(|s| s.split(" ").last().unwrap().to_string().parse::<i64>().unwrap()).collect_vec();
    let registers = [temp[0], temp[1], temp[2]];

    let program_string = parts.next().unwrap().split(" ").skip(1).next().unwrap();
    let program = program_string.split(",").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let result1 = run_program(registers, &program);

    let result2 = find_recursive(pow(2, 45), 13, &program); // found 2^45 by trial and error by checking correct length of response.

    (result1.iter().join(","),result2.to_string())
}


fn find_recursive(mut current: i64, exp: usize, goal: &Vec<i64>) -> i64 {
    for _ in 0..8 {
        let result = run_program([current, 0, 0], &goal);
        if result[exp] == goal[exp] {
            if exp== 0 {
                return current;
            }
            let recursive_result = find_recursive( current, exp-1, goal);
            if  recursive_result> 0 {
                return recursive_result;
            }
        }
        current += pow(8,exp);
    }
    0
}

fn run_program(mut registers: [i64; 3], program: &Vec<i64>) -> Vec<i64> {
    let mut pointer = 0;
    let mut output = vec![];

    while pointer < program.len() {
        let opcode = program[pointer];
        let operand = program[pointer+1];
        match opcode {
            0 => {registers[0] = dv(registers, operand); pointer += 2;},
            1 => {registers[1] = registers[1]^operand; pointer += 2;},
            2 => {registers[1] = combo_operand(registers, operand)%8; pointer += 2;},
            3 => {if registers[0]>0 {pointer = operand as usize} else { pointer += 2}},
            4 => {registers[1] = registers[1]^registers[2]; pointer += 2;},
            5 => {output.push(combo_operand(registers, operand)%8); pointer += 2;},
            6 => {registers[1] = dv(registers, operand); pointer += 2;}
            7 => {registers[2] = dv(registers, operand); pointer += 2;}
            _ => println!("Invalid opcode: {}", opcode)
        }
    }
    output
}


fn dv(registers: [i64; 3], operand: i64) -> i64 {
    let numerator = registers[0];
    let denominator = pow(2, combo_operand(registers, operand) as usize);
    numerator / denominator
}

fn combo_operand(registers: [i64; 3], operand: i64) -> i64 {
    match operand {
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        7 => panic!("combo operand 7 used"),
        _ => operand
    }
}