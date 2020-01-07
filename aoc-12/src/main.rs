use aoc_util::LCM;
use std::cmp::Ordering;
use std::fs;

fn main() {
    let input = fs::read_to_string("aoc-12/input.txt").unwrap();
    let mut system = MoonSystem::from_string(&input);

    system.step_until(|x| x.step_count == 1000);
    let energy = system.total_energy();
    println!("Total energy: {}", energy);

    system.reset();
    system.step_until(|s| s.is_initial_state()[0]);
    let x_period = system.step_count;
    system.reset();
    system.step_until(|s| s.is_initial_state()[1]);
    let y_period = system.step_count;
    system.reset();
    system.step_until(|s| s.is_initial_state()[2]);
    let z_period = system.step_count;
    println!(
        "x_period: {}, y_period: {}, z_period: {}",
        x_period, y_period, z_period
    );
    println!("lcm: {}", x_period.lcm(y_period).lcm(z_period));
}

type Coord3 = [i16; 3];

trait Coords {
    fn adjust(&mut self, adjustment: Coord3);
    fn get_energy(&self) -> i16;
}

impl Coords for Coord3 {
    fn adjust(&mut self, adjustment: Coord3) {
        for i in 0..self.len() {
            self[i] += adjustment[i];
        }
    }

    fn get_energy(&self) -> i16 {
        self.iter().fold(0, |acc, v| acc + v.abs())
    }
}

#[derive(Copy, Clone)]
pub struct Velocity(Coord3);

#[derive(Copy, Clone)]
pub struct Position(Coord3);

impl Position {
    fn calc_velocity_adjustment(&self, other: &Position) -> Coord3 {
        let mut result = [0; 3];
        for i in 0..self.0.len() {
            result[i] = match self.0[i].partial_cmp(&other.0[i]).unwrap() {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            }
        }
        result
    }
}

pub struct MoonSystem {
    moons: Vec<Moon>,
    step_count: u64,
}

impl MoonSystem {
    pub fn new() -> Self {
        MoonSystem {
            moons: Vec::new(),
            step_count: 0,
        }
    }

    pub fn from_string(input: &str) -> Self {
        let input = input
            .replace("<x=", "")
            .replace(" y=", "")
            .replace(" z=", "")
            .replace(">", "");
        let mut system = MoonSystem::new();
        for line in input.lines() {
            let mut coords = line.split(",").map(|x| x.parse::<i16>().unwrap());
            let pos = Position([
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            ]);
            let moon = Moon::new(pos);
            system.add_moon(moon);
        }
        system
    }

    pub fn add_moon(&mut self, moon: Moon) {
        self.moons.push(moon);
    }

    pub fn step_until<F>(&mut self, f: F)
    where
        F: Fn(&Self) -> bool,
    {
        loop {
            self.step();
            if f(self) == true {
                break;
            }
        }
    }

    pub fn total_energy(&self) -> i16 {
        self.moons.iter().fold(0, |acc, v| acc + v.get_energy())
    }

    pub fn reset(&mut self) {
        self.step_count = 0;
        for moon in self.moons.iter_mut() {
            moon.reset();
        }
    }

    fn step(&mut self) {
        self.apply_gravity();
        self.apply_velocity();
        self.step_count += 1;
    }

    fn apply_gravity(&mut self) {
        let positions: Vec<Position> = self
            .moons
            .iter()
            .map(|x| x.get_position().clone())
            .collect();
        for moon in self.moons.iter_mut() {
            moon.apply_gravity(&positions);
        }
    }

    fn apply_velocity(&mut self) {
        for moon in self.moons.iter_mut() {
            moon.apply_velocity();
        }
    }

    fn is_initial_state(&self) -> [bool; 3] {
        self.moons.iter().fold([true, true, true], |acc, m| {
            let [x, y, z] = m.is_initial_state();
            [acc[0] && x, acc[1] && y, acc[2] && z]
        })
    }
}

pub struct Moon {
    position: Position,
    velocity: Velocity,
    initial: Position,
}

impl Moon {
    pub fn new(pos: Position) -> Self {
        Moon {
            position: pos,
            velocity: Velocity([0; 3]),
            initial: pos.clone(),
        }
    }

    pub fn apply_gravity(&mut self, positions: &Vec<Position>) {
        let adjustment: Velocity = positions.iter().fold(Velocity([0; 3]), |mut acc, a| {
            acc.0.adjust(self.position.calc_velocity_adjustment(a));
            acc
        });
        self.velocity.0.adjust(adjustment.0);
    }

    pub fn apply_velocity(&mut self) {
        self.position.0.adjust(self.velocity.0);
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_velocity(&self) -> &Velocity {
        &self.velocity
    }

    pub fn get_energy(&self) -> i16 {
        self.position.0.get_energy() * self.velocity.0.get_energy()
    }

    pub fn is_initial_state(&self) -> [bool; 3] {
        let p = self.position.0;
        let i = self.initial.0;
        let v = self.velocity.0;
        [
            p[0] == i[0] && v[0] == 0,
            p[1] == i[1] && v[1] == 0,
            p[2] == i[2] && v[2] == 0,
        ]
    }

    pub fn reset(&mut self) {
        self.velocity = Velocity([0; 3]);
        self.position = self.initial.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_velocity_adjust() {
        let mut v = Velocity([0; 3]);
        v.0.adjust([-3, 0, 1]);
        assert_eq!([-3, 0, 1], v.0);
        v.0.adjust([-3, 0, 1]);
        assert_eq!([-6, 0, 2], v.0);
        v.0.adjust([-3, 999, -1]);
        assert_eq!([-9, 999, 1], v.0);
    }

    #[test]
    fn test_calc_velocity_adjust() {
        let v = Position([0; 3]);
        let r = v.calc_velocity_adjustment(&Position([-3, 0, 1]));
        assert_eq!([-1, 0, 1], r);
        let v = Position([-4, 1, 11]);
        let r = v.calc_velocity_adjustment(&Position([-3, 0, 1]));
        assert_eq!([1, -1, -1], r);
    }
}
