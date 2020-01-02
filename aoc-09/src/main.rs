use intcode::IntCode;
use std::fs;

fn main() {
    let input = fs::read_to_string("aoc-09/input.txt").unwrap();
    let mut processor = IntCode::from_string(&input);
    let mut processor2 = processor.clone();

    processor.input(&vec![1]);
    let result = processor.execute();
    println!("Test mode: {:?}", result);

    processor2.input(&vec![2]);
    let result = processor2.execute();
    println!("Sensor boost mode: {:?}", result);
}
