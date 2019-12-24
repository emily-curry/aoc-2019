use intcode::IntCode;
use std::fs;

const TARGET: usize = 19690720;

fn main() {
    let input = fs::read_to_string("aoc-02/input.txt").unwrap();
    part_01(&input);
    part_02(&input);
}

fn part_01(input: &String) {
    let mut data: Vec<usize> = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    data[1] = 12;
    data[2] = 2;
    let mut processor = IntCode::from_vec(data);
    println!("Part 1 answer: {}", processor.execute().unwrap());
}

fn part_02(input: &String) {
    let raw: Vec<usize> = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut input1: usize = 0;
    let mut input2: usize = 0;
    loop {
        let mut data = raw.clone();
        data[1] = input1;
        data[2] = input2;
        println!("Part 2 trying input: {}, {}", input1, input2);
        let mut processor = IntCode::from_vec(data);
        let result = processor.execute();
        if result.is_ok() && result.unwrap() == TARGET {
            break;
        }
        if input1 >= 100 {
            input1 = 0;
            input2 += 1;
        } else {
            input1 += 1;
        }
    }

    println!("Part 2 answer: {}, {}", input1, input2);
}
