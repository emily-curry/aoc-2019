pub struct IntCode {
    data: Vec<usize>,
    index: usize,
}

impl IntCode {
    pub fn from_vec(raw: Vec<usize>) -> IntCode {
        IntCode {
            data: raw,
            index: 0,
        }
    }

    pub fn from_string(raw: String) -> IntCode {
        let data: Vec<usize> = raw
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        IntCode { data, index: 0 }
    }

    pub fn execute(&mut self) -> usize {
        loop {
            match self.operation() {
                Operation::Add => self.exec_add(),
                Operation::Multiply => self.exec_multiply(),
                Operation::Halt => break,
            }
            self.advance();
        }

        self.data[0]
    }

    pub fn setup_1202(&mut self) {
        self.data[1] = 12;
        self.data[2] = 2;
    }

    fn operation(&self) -> Operation {
        match self.data[self.index] {
            1 => Operation::Add,
            2 => Operation::Multiply,
            99 => Operation::Halt,
            _ => panic!("No operation for code"),
        }
    }

    fn advance(&mut self) {
        match self.operation() {
            Operation::Add | Operation::Multiply => self.index += 4,
            Operation::Halt => self.index += 1,
        }
    }

    fn exec_add(&mut self) {
        let val = self.data[self.data[self.index + 1]] + self.data[self.data[self.index + 2]];
        let out = self.data[self.index + 3];
        println!("Operation::Add: Storing {} in index {}", val, out);
        self.data[out] = val;
    }

    fn exec_multiply(&mut self) {
        let val = self.data[self.data[self.index + 1]] * self.data[self.data[self.index + 2]];
        let out = self.data[self.index + 3];
        println!("Operation::Multiply: Storing {} in index {}", val, out);
        self.data[out] = val;
    }
}

enum Operation {
    Add,
    Multiply,
    Halt,
}
