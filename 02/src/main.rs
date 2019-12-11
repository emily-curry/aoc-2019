struct IntCode {
    data: Vec<usize>,
    index: usize,
}

impl IntCode {
    fn from(raw: String) -> IntCode {
        let data: Vec<usize> = raw
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        IntCode { data, index: 0 }
    }

    fn should_continue(&self) -> bool {
        match self.data[self.index] {
            1 | 2 => true,
            99 => false,
            _ => panic!(),
        }
    }

    fn execute(&mut self) {
        let (opcode, v1, v2, out) = self.ops();
        println!(
            "Index {}, opcode {}, v1 {}, v2 {}, out {}",
            self.index, opcode, v1, v2, out
        );
        match opcode {
            1 => {
                let val = v1 + v2;
                println!("Storing {} in index {}", val, out);
                self.data[out] = val;
            }
            2 => {
                let val = v1 * v2;
                println!("Storing {} in index {}", val, out);
                self.data[out] = val;
            }
            _ => panic!(),
        }
    }

    fn ops(&self) -> (usize, usize, usize, usize) {
        (
            self.data[self.index],
            self.data[self.data[self.index + 1]],
            self.data[self.data[self.index + 2]],
            self.data[self.index + 3],
        )
    }

    fn step(&mut self) {
        self.index += 4;
    }

    fn setup_1202(&mut self) {
        self.data[1] = 12;
        self.data[2] = 2;
    }
}

fn main() {
    use std::fs;
    let input = fs::read_to_string("input.txt").unwrap();
    let mut processor = IntCode::from(input);
    println!("{:?}", processor.data);
    processor.setup_1202();
    while processor.should_continue() {
        processor.execute();
        processor.step();
    }
    println!("Position 0: {}", processor.data[0]);
}
