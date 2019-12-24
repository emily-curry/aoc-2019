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

    pub fn execute(&mut self) -> Result<usize, ()> {
        loop {
            let result = match self.operation() {
                Operation::Add => self.exec_add(),
                Operation::Multiply => self.exec_multiply(),
                Operation::Halt => break,
            };
            if let Err(e) = result {
                return Err(e);
            };
            self.advance();
        }

        Ok(self.data[0])
    }

    fn operation(&self) -> Operation {
        match self.data[self.index] {
            1 => Operation::Add,
            2 => Operation::Multiply,
            99 => Operation::Halt,
            _ => panic!("No operation for code"),
        }
    }

    fn operation_length(&self) -> usize {
        match self.operation() {
            Operation::Add | Operation::Multiply => 4,
            Operation::Halt => 1,
        }
    }

    fn advance(&mut self) {
        self.index += self.operation_length();
    }

    fn is_safe(&self, index: usize) -> bool {
        index <= self.data.len() - 1
    }

    fn exec_add(&mut self) -> Result<(), ()> {
        if !self.is_safe(self.index + self.operation_length())
            || !self.is_safe(self.data[self.index + 1])
            || !self.is_safe(self.data[self.index + 2])
            || !self.is_safe(self.data[self.index + 3])
        {
            return Err(());
        }

        let val = self.data[self.data[self.index + 1]] + self.data[self.data[self.index + 2]];
        let out = self.data[self.index + 3];
        // println!("Operation::Add: Storing {} in index {}", val, out);
        self.data[out] = val;
        Ok(())
    }

    fn exec_multiply(&mut self) -> Result<(), ()> {
        if !self.is_safe(self.index + self.operation_length())
            || !self.is_safe(self.data[self.index + 1])
            || !self.is_safe(self.data[self.index + 2])
            || !self.is_safe(self.data[self.index + 3])
        {
            return Err(());
        }

        let val = self.data[self.data[self.index + 1]] * self.data[self.data[self.index + 2]];
        let out = self.data[self.index + 3];
        // println!("Operation::Multiply: Storing {} in index {}", val, out);
        self.data[out] = val;
        Ok(())
    }
}

enum Operation {
    Add,
    Multiply,
    Halt,
}
