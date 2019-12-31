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

    pub fn from_string(raw: &String) -> IntCode {
        let data: Vec<isize> = raw
            .split(",")
            .map(|x| x.parse::<isize>().unwrap())
            .collect();
        IntCode { data, index: 0 }
    }

    pub fn execute(&mut self, inputs: Vec<isize>) -> Result<(isize, Vec<isize>), ()> {
        let mut input_iter = inputs.iter();
        let mut outputs = Vec::new();
        loop {
            let result = match self.operation() {
                Operation::Add(i) => self.exec_add(&i),
                Operation::Multiply(i) => self.exec_multiply(&i),
                Operation::Input => self.exec_input(*input_iter.next().unwrap()),
                Operation::Output(i) => self.exec_output(&i),
                Operation::JumpIfTrue(i) => self.exec_jump_if_true(&i),
                Operation::JumpIfFalse(i) => self.exec_jump_if_false(&i),
                Operation::LessThan(i) => self.exec_less_than(&i),
                Operation::Equals(i) => self.exec_equals(&i),
                Operation::Halt => break,
            }?;
            if let Some(i) = result.output {
                outputs.push(i);
            }
            if result.advance {
                self.advance();
            }
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

        let el0 = OperationMode::from(params[0]);
        let el1 = OperationMode::from(params[1]);
        match op {
            1 => Operation::Add([el0, el1]),
            2 => Operation::Multiply([el0, el1]),
            3 => Operation::Input,
            4 => Operation::Output([el0]),
            5 => Operation::JumpIfTrue([el0, el1]),
            6 => Operation::JumpIfFalse([el0, el1]),
            7 => Operation::LessThan([el0, el1]),
            8 => Operation::Equals([el0, el1]),
            99 => Operation::Halt,
            _ => panic!("No operation for code: {}", op),
        }
    }

    fn operation_length(&self) -> usize {
        match self.operation() {
            Operation::Add(_)
            | Operation::Multiply(_)
            | Operation::LessThan(_)
            | Operation::Equals(_) => 4,
            Operation::JumpIfTrue(_) | Operation::JumpIfFalse(_) => 3,
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

    fn exec_add(&mut self, modes: &[OperationMode; 2]) -> Result<OperationResult, ()> {
        let val = self.read(self.index + 1, &modes[0])? + self.read(self.index + 2, &modes[1])?;
        self.write(self.index + 3, val)?;
        Ok(Default::default())
    }

    fn exec_multiply(&mut self, modes: &[OperationMode; 2]) -> Result<OperationResult, ()> {
        let val = self.read(self.index + 1, &modes[0])? * self.read(self.index + 2, &modes[1])?;
        self.write(self.index + 3, val)?;
        Ok(Default::default())
    }

    fn exec_input(&mut self, input: isize) -> Result<OperationResult, ()> {
        self.write(self.index + 1, input)?;
        Ok(Default::default())
    }

    fn exec_output(&self, modes: &[OperationMode; 1]) -> Result<OperationResult, ()> {
        let result = self.read(self.index + 1, &modes[0])?;
        Ok(OperationResult {
            output: Some(result),
            ..Default::default()
        })
    }

    fn exec_jump_if_true(&mut self, modes: &[OperationMode; 2]) -> Result<OperationResult, ()> {
        let val = self.read(self.index + 1, &modes[0])?;
        if val != 0 {
            let next_index = self.read(self.index + 2, &modes[1])?;
            self.index = next_index as usize;
            return Ok(OperationResult {
                advance: false,
                ..Default::default()
            });
        }
        Ok(Default::default())
    }

    fn exec_jump_if_false(&mut self, modes: &[OperationMode; 2]) -> Result<OperationResult, ()> {
        let val = self.read(self.index + 1, &modes[0])?;
        if val == 0 {
            let next_index = self.read(self.index + 2, &modes[1])?;
            self.index = next_index as usize;
            return Ok(OperationResult {
                advance: false,
                ..Default::default()
            });
        }
        Ok(Default::default())
    }

    fn exec_less_than(&mut self, modes: &[OperationMode; 2]) -> Result<OperationResult, ()> {
        let left = self.read(self.index + 1, &modes[0])?;
        let right = self.read(self.index + 2, &modes[1])?;
        let val = match left < right {
            true => 1,
            false => 0,
        };
        self.write(self.index + 3, val)?;
        Ok(Default::default())
    }

    fn exec_equals(&mut self, modes: &[OperationMode; 2]) -> Result<OperationResult, ()> {
        let left = self.read(self.index + 1, &modes[0])?;
        let right = self.read(self.index + 2, &modes[1])?;
        let val = match left == right {
            true => 1,
            false => 0,
        };
        self.write(self.index + 3, val)?;
        Ok(Default::default())
    }
}

impl Clone for IntCode {
    fn clone(&self) -> IntCode {
        IntCode {
            data: self.data.clone(),
            index: self.index,
        }
    }
}

enum Operation {
    Add([OperationMode; 2]),
    Multiply([OperationMode; 2]),
    Input,
    Output([OperationMode; 1]),
    JumpIfTrue([OperationMode; 2]),
    JumpIfFalse([OperationMode; 2]),
    LessThan([OperationMode; 2]),
    Equals([OperationMode; 2]),
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

struct OperationResult {
    advance: bool,
    output: Option<isize>,
}

impl Default for OperationResult {
    fn default() -> Self {
        OperationResult {
            advance: true,
            output: None,
        }
    }
}
