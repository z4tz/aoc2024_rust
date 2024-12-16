use std::iter::Cycle;
use std::slice::Iter;
use crate::Solution;
use std::time::{Duration, Instant};

pub struct Day09 {}

impl Solution for Day09 {
    fn timed_solution(&self, data: &str) -> (String, String, Duration) {
        let start = Instant::now(); // skip file IO in timing
        let result1 = filesystem_checksum(data);
        let result2 = filesystem_checksum2(data);
        let duration = start.elapsed();
        (result1, result2, duration)
    }
}

fn filesystem_checksum(data: &str) -> String {
    let numbers = data.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
    let mut reverse_files = vec![];
    for (i, file) in numbers.iter().step_by(2).enumerate().rev() {
        for _ in 0..*file {
            reverse_files.push(i as i32);
        }
    }
    let mut files = vec![];
    for (i, file) in numbers.iter().step_by(2).enumerate() {
        for _ in 0..*file {
            files.push(i as i32);
        }
    }
    let max_count = files.len() as i32-1;
    let mut count:i32 = 0;
    let mut numbers_iter = numbers.iter();
    let mut iterators = [files.iter(), reverse_files.iter()];
    let mut indicies:Cycle<Iter<usize>>  = [0, 1].iter().cycle();

    let mut checksum:i64 = 0;
    'outer: loop {
        let iterator = &mut iterators[*indicies.next().unwrap()];

        for _ in 0..*numbers_iter.next().unwrap() {
            let num = *iterator.next().unwrap();
            checksum += (num * count) as i64;
            if count >= max_count{
                break 'outer;
            }
            count += 1;
        }
    }

    checksum.to_string()
}

#[derive(Clone)]
struct DiscSection {
    size: usize,
    id: Option<usize>,
    offset: usize
}
impl DiscSection {
    fn empty(size: usize, offset: usize) -> DiscSection {
        DiscSection { size, id: None, offset}
        }
    fn new( size: usize, id: usize, offset: usize) -> DiscSection {
        DiscSection { size, id: Some(id), offset}
    }
}


fn filesystem_checksum2(data: &str) -> String {
    let numbers = data.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
    let mut files: Vec<DiscSection> = vec![];
    let mut spaces: Vec<DiscSection> = vec![];
    let mut offset = 0;
    for (i, size) in numbers.iter().enumerate() {
        if i%2 == 0 {  //add file
            files.push(DiscSection::new(*size, i/2, offset))
        }
        else { spaces.push(DiscSection::empty(*size, offset)) }
        offset += size;
    }
    let mut cache: [usize;10] = [0;10];
    let mut checksum= 0;
    for (index,item) in files.iter().enumerate().rev() {
        let size = item.size;
        let mut moved = false;
        if cache[size] <= index {  // skip in cases where cached index is larger than current index
            for (space_index, possible_space) in spaces[cache[size]..index].iter_mut().enumerate() {
                if possible_space.size >= size {
                    checksum += item.id.unwrap() * size * ( 2 * possible_space.offset + size-1)/2 ;  // s = n(a+l)/2

                    possible_space.size -= size;
                    possible_space.offset += size;
                    moved = true;
                    cache[size] = space_index;
                    break;
                }
            }
        }
        if !moved {
            checksum += item.id.unwrap() * size * ( 2 * item.offset + size-1)/2;  // s = n(a+l)/2

        }
    }
    checksum.to_string()
}
