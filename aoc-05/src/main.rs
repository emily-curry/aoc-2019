use intcode::IntCode;
use std::fs;

fn main() {
    let input = fs::read_to_string("aoc-05/input.txt").unwrap();
    let mut processor = IntCode::from_string(&input);
    processor.input(&vec![1]);
    let result = processor.execute().unwrap();
    println!("Part 1: {:?}", result.output);
    let mut processor2 = IntCode::from_string(&input);
    processor2.input(&vec![5]);
    let result2 = processor2.execute().unwrap();
    println!("Part 2: {:?}", result2.output);
}
