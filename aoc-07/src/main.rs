use aoc_util::Permutation;
use intcode::{IntCode, IntCodeResultKind};
use std::fs;

fn main() {
    let input = fs::read_to_string("aoc-07/input.txt").unwrap();
    series(&input);
    feedback(&input);
}

fn series(input: &str) {
    let permutations: Vec<Vec<isize>> = Permutation::calculate(&vec![0, 1, 2, 3, 4]);
    let mut largest: isize = 0;

    for p in permutations {
        let mut c = spawn_cpu(input);
        let mut out: isize = 0;

        for (idx, p) in p.iter().enumerate() {
            let cpu = c.get_mut(idx).unwrap();
            cpu.input(&vec![*p, out]);
            let result = cpu.execute().unwrap();
            out = result.output[0];
        }

        if out > largest {
            largest = out;
        }
    }

    println!("Series - Max value: {:?}", largest);
}

fn feedback(input: &str) {
    let permutations: Vec<Vec<isize>> = Permutation::calculate(&vec![9, 8, 7, 6, 5]);
    let mut largest: isize = 0;

    for p in permutations {
        let mut c = spawn_cpu(input);

        for (idx, p) in p.iter().enumerate() {
            let cpu = c.get_mut(idx).unwrap();
            cpu.input(&vec![*p]);
        }

        let mut out: isize = 0;
        let mut i = 0;

        loop {
            let idx = i % 5;
            let cpu = c.get_mut(idx).unwrap();
            cpu.input(&vec![out]);
            let result = cpu.execute().unwrap();
            out = *result.output.last().unwrap();
            match result.kind {
                IntCodeResultKind::Halt => {
                    if idx == 4 {
                        break;
                    }
                }
                IntCodeResultKind::Yield => {}
            }
            i += 1;
        }

        if out > largest {
            largest = out;
        }
    }

    println!("Feedback - Max value: {:?}", largest);
}

fn spawn_cpu(input: &str) -> [IntCode; 5] {
    let cpu = IntCode::from_string(input);
    [cpu.clone(), cpu.clone(), cpu.clone(), cpu.clone(), cpu]
}
