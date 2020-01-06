use aoc_util::Direction;
use intcode::{IntCode, IntCodeResultKind};
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("aoc-11/input.txt").unwrap();
    let mut robot = HullPaintRobot::new(&input);
    let paint = robot.execute();
    println!("Tiles painted: {}", paint.len());
}

pub enum HullPaintColor {
    Black,
    White,
}

impl From<isize> for HullPaintColor {
    fn from(int: isize) -> Self {
        match int {
            0 => HullPaintColor::Black,
            1 => HullPaintColor::White,
            _ => panic!("No paint color for {}", int),
        }
    }
}

impl From<&HullPaintColor> for isize {
    fn from(c: &HullPaintColor) -> Self {
        match c {
            HullPaintColor::Black => 0,
            HullPaintColor::White => 1,
        }
    }
}

pub struct HullPaintRobot {
    painted: HashMap<(i8, i8), HullPaintColor>,
    location: (i8, i8),
    direction: Direction,
    processor: IntCode,
}

impl HullPaintRobot {
    pub fn new(program: &str) -> Self {
        HullPaintRobot {
            painted: HashMap::new(),
            location: (0, 0),
            direction: Direction::Up,
            processor: IntCode::from_string(program),
        }
    }

    pub fn execute(&mut self) -> &HashMap<(i8, i8), HullPaintColor> {
        let mut out_idx = 0;
        loop {
            let input: isize = self.color_at_location().into();
            self.processor.input(&vec![input]);
            let result = self.processor.execute().unwrap();
            let o1 = result.output[out_idx];
            let o2 = result.output[out_idx + 1];
            let k = result.kind;
            self.paint(o1);
            self.rotate(o2);
            self.advance();
            out_idx = out_idx + 2;
            if let IntCodeResultKind::Halt = k {
                break;
            }
        }
        &self.painted
    }

    fn color_at_location(&self) -> &HullPaintColor {
        self.painted
            .get(&self.location)
            .unwrap_or(&HullPaintColor::Black)
    }

    fn paint(&mut self, input: isize) {
        self.painted
            .insert(self.location, HullPaintColor::from(input));
    }

    fn rotate(&mut self, input: isize) {
        self.direction = match input {
            0 => self.direction.counter_clockwise(),
            1 => self.direction.clockwise(),
            _ => panic!(),
        };
    }

    fn advance(&mut self) {
        self.location = match self.direction {
            Direction::Up => (self.location.0, self.location.1 - 1),
            Direction::Down => (self.location.0, self.location.1 + 1),
            Direction::Left => (self.location.0 - 1, self.location.1),
            Direction::Right => (self.location.0 + 1, self.location.1),
        }
    }
}
