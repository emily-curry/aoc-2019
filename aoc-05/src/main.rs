use intcode::IntCode;
use std::fs;

fn main() {
    let input = fs::read_to_string("aoc-05/input.txt").unwrap();
    let mut processor = IntCode::from_string(&input);
    let result = processor.execute(vec![1]).unwrap();
    println!("Part 1: {:?}", result);
    let mut processor2 = IntCode::from_string(&input);
    let result2 = processor2.execute(vec![5]).unwrap();
    println!("Part 2: {:?}", result2);
}
