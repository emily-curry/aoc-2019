use aoc_util::digits;

pub struct IntCode {
    data: Vec<isize>,
    index: usize,
}

impl IntCode {
    pub fn from_vec(raw: Vec<isize>) -> IntCode {
        IntCode {
            data: raw,
            index: 0,
        }
    }

    pub fn from_string(raw: String) -> IntCode {
        let data: Vec<isize> = raw
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        IntCode { data, index: 0 }
    }

    pub fn execute(&mut self, inputs: Vec<isize>) -> Result<(isize, Vec<isize>), ()> {
        let mut outputs = Vec::new();
        loop {
            let result = match self.operation() {
                Operation::Add(i) => self.exec_add(&i),
                Operation::Multiply(i) => self.exec_multiply(&i),
                Operation::Input => self.exec_input(inputs[0]),
                Operation::Output(i) => {
                    outputs.push(self.exec_output(&i)?);
                    Ok(())
                }
                Operation::Halt => break,
            };
            if let Err(e) = result {
                return Err(e);
            };
            self.advance();
        }

        Ok((self.data[0], outputs))
    }

    fn operation(&self) -> Operation {
        let opcode = digits(self.data[self.index]);
        let op = opcode[0] + (opcode.get(1).cloned().unwrap_or(0) * 10);
        let mut params = [0; 2];
        for i in 0..params.len() {
            params[i] = opcode.get(i + 2).cloned().unwrap_or(0);
        }

        match op {
            1 => Operation::Add([
                OperationMode::from(params[0]),
                OperationMode::from(params[1]),
            ]),
            2 => Operation::Multiply([
                OperationMode::from(params[0]),
                OperationMode::from(params[1]),
            ]),
            3 => Operation::Input,
            4 => Operation::Output([OperationMode::from(params[0])]),
            99 => Operation::Halt,
            _ => panic!("No operation for code: {}", op),
        }
    }

    fn operation_length(&self) -> usize {
        match self.operation() {
            Operation::Add(_) | Operation::Multiply(_) => 4,
            Operation::Input | Operation::Output(_) => 2,
            Operation::Halt => 1,
        }
    }

    fn advance(&mut self) {
        self.index += self.operation_length();
    }

    fn is_safe(&self, index: isize) -> bool {
        index >= 0 && index as usize <= self.data.len() - 1
    }

    fn read(&self, index: usize, mode: &OperationMode) -> Result<isize, ()> {
        match mode {
            OperationMode::Position => {
                if !self.is_safe(index as isize) || !self.is_safe(self.data[index]) {
                    return Err(());
                }
                Ok(self.data[self.data[index] as usize])
            }
            OperationMode::Immediate => {
                if !self.is_safe(index as isize) {
                    return Err(());
                }
                Ok(self.data[index])
            }
        }
    }

    fn write(&mut self, index: usize, value: isize) -> Result<(), ()> {
        if !self.is_safe(index as isize) || !self.is_safe(self.data[index]) {
            return Err(());
        }
        let out = self.data[index] as usize;
        self.data[out] = value;
        Ok(())
    }

    fn exec_add(&mut self, modes: &[OperationMode; 2]) -> Result<(), ()> {
        let val = self.read(self.index + 1, &modes[0])? + self.read(self.index + 2, &modes[1])?;
        self.write(self.index + 3, val)?;
        Ok(())
    }

    fn exec_multiply(&mut self, modes: &[OperationMode; 2]) -> Result<(), ()> {
        let val = self.read(self.index + 1, &modes[0])? * self.read(self.index + 2, &modes[1])?;
        self.write(self.index + 3, val)?;
        Ok(())
    }

    fn exec_input(&mut self, input: isize) -> Result<(), ()> {
        self.write(self.index + 1, input)?;
        Ok(())
    }

    fn exec_output(&self, modes: &[OperationMode; 1]) -> Result<isize, ()> {
        let result = self.read(self.index + 1, &modes[0])?;
        Ok(result)
    }
}

enum Operation {
    Add([OperationMode; 2]),
    Multiply([OperationMode; 2]),
    Input,
    Output([OperationMode; 1]),
    Halt,
}

enum OperationMode {
    Position,
    Immediate,
}

impl OperationMode {
    fn from(i: isize) -> OperationMode {
        match i {
            0 => OperationMode::Position,
            1 => OperationMode::Immediate,
            _ => panic!("No operation mode: {}", i),
        }
    }
}
