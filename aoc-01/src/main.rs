use std::fs;

const INCLUDE_FUEL_MASS: bool = true;

fn main() {
    let input = fs::read_to_string("aoc-01/input.txt").unwrap();
    let lines = input.split("\n");
    let mut total_fuel = 0.0;
    for line in lines {
        let mass = line.parse::<f64>().unwrap();
        total_fuel += compute_fuel(mass);
    }
    println!("{}", total_fuel);
}

fn compute_fuel(mass: f64) -> f64 {
    let fuel = fuel_for_mass(mass);
    if INCLUDE_FUEL_MASS && fuel_for_mass(fuel) > 0.0 {
        fuel + compute_fuel(fuel)
    } else {
        fuel
    }
}

fn fuel_for_mass(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}
