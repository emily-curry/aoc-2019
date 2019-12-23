use intcode;

fn main() {
    use std::fs;
    let input = fs::read_to_string("aoc-02/input.txt").unwrap();
    let mut data: Vec<usize> = input
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    data[1] = 12;
    data[2] = 2;
    let mut processor = intcode::IntCode::from_vec(data);
    println!("Position 0: {}", processor.execute());
}
