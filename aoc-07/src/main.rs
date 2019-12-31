use aoc_util::permutation::Permutation;
use intcode::IntCode;
use std::fs;

fn main() {
    let input = fs::read_to_string("aoc-07/input.txt").unwrap();
    let permutations: Vec<Vec<isize>> = Permutation::calculate(&vec![0, 1, 2, 3, 4]);
    let mut largest: isize = 0;
    for p in permutations {
        let mut c = spawn_cpu(&input);
        let mut out: isize = 0;
        for (idx, p) in p.iter().enumerate() {
            let cpu = c.get_mut(idx).unwrap();
            let (_, cpu_out) = cpu.execute(vec![*p, out]).unwrap();
            out = cpu_out[0];
        }
        if out > largest {
            largest = out;
        }
    }

    println!("Max value: {:?}", largest);
}

fn spawn_cpu(input: &String) -> [IntCode; 5] {
    let cpu = IntCode::from_string(input);
    [cpu.clone(), cpu.clone(), cpu.clone(), cpu.clone(), cpu]
}
