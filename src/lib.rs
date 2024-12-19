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