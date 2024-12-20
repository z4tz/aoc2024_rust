use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use crate::Solution;
use std::time::{Duration, Instant};
use num::Complex;

pub struct Day16 {}

impl Solution for Day16 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let (result1, result2) = mazing(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn mazing(data: &str) -> (String, String) {
    let mut tiles = HashSet::new();
    let mut state: State = State::new(Complex::new(0,0), Complex::new(0,0));
    let mut end = Complex::new(0, 0);

    for (y, line) in data.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' { tiles.insert(Complex::new(x as i32, y as i32));}
            if ch == 'S' { state = State::new(Complex::new(x as i32, y as i32), Complex::new(1, 0));}
            if ch == 'E' {
                end = Complex::new(x as i32, y as i32);
                tiles.insert(Complex::new(x as i32, y as i32));
            }
        }
    }

    let mut best_paths: HashMap<State, BestPath> = HashMap::new();
    let mut visited = HashSet::new();
    let mut to_visit = vec![];
    let final_score;
    let final_state;  // for backtracking
    to_visit.push((0, state, None));
    loop {
        to_visit.sort_by_key(|scorestate| Reverse(scorestate.0));
        let (score, state, previous_state) = to_visit.pop().unwrap();

        //check if we've been here already with the same score, then add parent path
        if best_paths.contains_key(&state) {
            let temp_best_path = best_paths.get_mut(&state).unwrap();
            if temp_best_path.score == score {
                temp_best_path.parents.push(previous_state.unwrap());
            }
        }
        else {
            let mut best_path = BestPath::new(state.clone(), score);
            if previous_state.is_some() {
                best_path.parents.push(previous_state.unwrap());
            }
            best_paths.insert(state.clone(),best_path);
        }

        if state.position == end {
            final_score = score;
            final_state = state;
            break;}

        for possible_direction in possible_directions(state.direction) {
            let tempstate = State::new(state.position + possible_direction, possible_direction);
            if tiles.contains(&(tempstate.position)){
                let tempscore = score + 1 + if state.direction != possible_direction { 1000 } else { 0 };

                if !visited.contains(&tempstate) && !to_visit.contains(&(tempscore, tempstate.clone(), Some(state.clone()))) {
                    to_visit.push((tempscore, tempstate, Some(state.clone())));
                }
                else {

                }
            }
        }
        visited.insert(state.clone());

    }
    // backtrack on best paths and count unique tiles
    let mut best_path_tiles = HashSet::new();
    let mut current_paths = vec![best_paths.get(&final_state).unwrap()];
    while !current_paths.is_empty() {
        let mut new_paths = vec![];
        for path in current_paths {
            best_path_tiles.insert(path.state.position);
            for parent in &path.parents {
                new_paths.push(best_paths.get(&parent).unwrap());
            }
        }
        current_paths = new_paths;
    }



    (final_score.to_string(), best_path_tiles.len().to_string())
}

fn possible_directions(current_direction: Complex<i32>) -> Vec<Complex<i32>> {
    vec![current_direction * Complex::i(), current_direction, current_direction * Complex::i() * Complex::i() * Complex::i() ]
}

#[derive(PartialEq, Eq)]
#[derive(Hash)]
#[derive(Clone)]
struct State {
    position : Complex<i32>,
    direction : Complex<i32>
}
impl State {
    fn new(position: Complex<i32>, direction: Complex<i32>) -> State { State { position, direction }}
}
impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "State: pos ({},{}) dir ({},{})",self.position.re,self.position.im,self.direction.re,self.direction.im)
    }
}

#[derive(Hash)]
#[derive(PartialEq, Eq, Debug)]
struct BestPath {
    state : State,
    score : i32,
    parents : Vec<State>
}
impl BestPath {
    fn new(state : State, score: i32) -> BestPath {BestPath{state, score, parents: vec![]}}
}