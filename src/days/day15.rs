use std::collections::{HashMap, HashSet};
use crate::{Solution, pause};
use std::time::{Duration, Instant};
use itertools::Itertools;
use ndarray::Array2;
use num::Complex;

pub struct Day15 {}

impl Solution for Day15 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let result1 = box_robot(data);
        let result2 = box_robot(&modify_data(data));
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn modify_data(data: &str) -> String {
    let modified = data
        .replace("#", "##")
        .replace(".","..")
        .replace("O", "[]")
        .replace("@", "@.");
    modified
}

#[derive(Debug)]
struct Boxx {
    positions: Vec<Complex<i32>>
}
impl Boxx {
    fn new(positions: Vec<Complex<i32>>) -> Boxx {
        Boxx { positions }
    }
}

fn box_robot(data: &str) -> String {
    let mut parts = data.split("\r\n\r\n");
    let board = parts.next().unwrap();
    let instructions = parts.next().unwrap().lines().join("");
    let mut walls = HashSet::new();
    let mut box_bindings= vec![];
    let mut boxes = HashMap::new();
    let mut position = Complex::new(0, 0);
    for (y, line) in board.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#'  {walls.insert(Complex { re: x as i32, im: y as i32 });}
            else if ch == 'O' {
                let position = Complex { re: x as i32, im: y as i32 };
                box_bindings.push(Boxx::new(vec![position]));
            }
            else if ch == '[' {
                let boxx = Boxx::new(vec![Complex { re: x as i32, im: y as i32 }, Complex { re: x as i32 + 1, im: y as i32 }]);
                box_bindings.push(boxx);
            }
            else if ch == '@' {position = Complex { re: x as i32, im: y as i32 };}
        }
    }
    for (index, boxx) in box_bindings.iter().enumerate() {
        for position in &boxx.positions {
            boxes.insert(*position, index);
        }
    }

    for instruction in instructions.chars() {
        let direction = directions(instruction);
        let temp_position = position + direction;
        if walls.contains(&temp_position) {  //wall
            continue;
        }
        else if boxes.contains_key(&temp_position) {  //box
            let box_index = *boxes.get(&temp_position).unwrap();
            let movable_boxes = try_move_box(box_index, direction, &walls, &boxes, &box_bindings);
            if !movable_boxes.is_empty() {

                for movable_box_index in HashSet::<usize>::from_iter(movable_boxes.clone()) {  // remove all positions first
                    let boxx = box_bindings.get_mut(movable_box_index).unwrap();
                    boxx.positions.iter().for_each(|position| {boxes.remove(position);});
                }
                for movable_box_index in HashSet::<usize>::from_iter(movable_boxes) {  // then add them
                    let boxx = box_bindings.get_mut(movable_box_index).unwrap();
                    boxx.positions.iter_mut().map(|position|*position += direction).collect_vec();
                    boxx.positions.iter().for_each(|position|{boxes.insert(*position,movable_box_index);});
                }
                position = position + direction;
            }
        }
        else {  //empty space
            position = position + direction;
        }
    }
    let gps_coordinates: i32 = box_bindings.iter()
        .map(|b| b.positions[0].re + b.positions[0].im * 100).sum();
    gps_coordinates.to_string()
}

fn directions(direction: char) -> Complex<i32> {
    match direction {
        '^' => Complex::new(0, -1),
        'v' => Complex::new(0, 1),
        '<' => Complex::new(-1, 0),
        '>' => Complex::new(1, 0),
        _ => Complex::new(0, 0),
    }
}

fn try_move_box(box_index: usize, direction: Complex<i32>, walls: &HashSet<Complex<i32>>, boxes: &HashMap<Complex<i32>, usize>, box_bindings: &Vec<Boxx>) -> Vec<usize> {
    let mut movable_boxes = vec![];
    let boxx = box_bindings.get(box_index).unwrap();
    for position in &boxx.positions {
        if walls.contains(&(position+direction)) {return vec![];}
        if !boxx.positions.contains(&(position + direction)) && boxes.contains_key(&(position+direction)){  // make sure it's not the same box
            let temp_box = *boxes.get(&(position + direction)).unwrap();
            let movable_boxes_temp = try_move_box(temp_box, direction, walls, boxes, box_bindings);
            if movable_boxes_temp.is_empty() {
                return movable_boxes_temp;
            }
            else {
                movable_boxes.extend(movable_boxes_temp);
            }
        }
    }
    movable_boxes.push(box_index);
    movable_boxes
}

fn print_state(position: &Complex<i32>, walls: &HashSet<Complex<i32>>, boxes: &Vec<Boxx>, dimensions: (usize, usize)) {
    let mut image = Array2::from_elem(dimensions, '.');
    for wall in walls {
        image[(wall.im as usize, wall.re as usize)] = '#';
    }
    for (index,boxx) in boxes.iter().enumerate() {  // only works for part 2, used index to keep track of boxes in image
        let number = format!("{:0>2}", index);
        image[(boxx.positions[0].im as usize, boxx.positions[0].re as usize)] = number.chars().nth(0).unwrap();
        image[(boxx.positions[1].im as usize, boxx.positions[1].re as usize)] = number.chars().nth(1).unwrap();
    }
    image[(position.im as usize, position.re as usize)] = '@';

    println!("{}", image.to_string().replace(", ",""));
    println!(" ");
    pause!();
}