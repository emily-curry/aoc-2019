use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("aoc-03/input.txt").unwrap();
    let lines = input.split("\n");
    let mut panel = WirePanel::new();
    for line in lines {
        panel.add_wire(Wire::from(line));
    }
    let nearest_collision = panel.get_nearest_collision();
    println!("Puzzle 3, part 1: {}", nearest_collision);

    let soonest_collision = panel.get_soonest_collision();
    println!("Puzzle 3, part 2: {}", soonest_collision);
}

struct WirePanel {
    wires: Vec<Wire>,
    nearest_collision: Option<i32>,
    soonest_collision: Option<i32>,
}

impl WirePanel {
    pub fn new() -> WirePanel {
        WirePanel {
            wires: Vec::new(),
            nearest_collision: None,
            soonest_collision: None,
        }
    }

    pub fn add_wire(&mut self, new_wire: Wire) {
        for wire in &self.wires {
            let collisions: HashSet<_> = wire.locs.intersection(&new_wire.locs).collect();
            for collision in collisions {
                let &(x, y) = collision;
                let dist = WirePanel::get_distance(x, y);
                match self.nearest_collision {
                    Some(i) => {
                        if dist < i {
                            self.nearest_collision = Some(dist)
                        }
                    }
                    None => self.nearest_collision = Some(dist),
                }

                let total_length = wire.length_at(x, y) + new_wire.length_at(x, y);
                match self.soonest_collision {
                    Some(i) => {
                        if total_length < i {
                            self.soonest_collision = Some(total_length)
                        }
                    }
                    None => self.soonest_collision = Some(total_length),
                }
            }
        }
        self.wires.push(new_wire);
    }

    pub fn get_nearest_collision(&self) -> i32 {
        match self.nearest_collision {
            Some(i) => i,
            None => panic!(),
        }
    }

    pub fn get_soonest_collision(&self) -> i32 {
        match self.soonest_collision {
            Some(i) => i,
            None => panic!(),
        }
    }

    fn get_distance(x: i32, y: i32) -> i32 {
        i32::abs(x) + i32::abs(y)
    }
}

struct Wire {
    pub locs: HashSet<(i32, i32)>,
    loc_step: HashMap<(i32, i32), i32>,
}

impl Wire {
    pub fn from(input: &str) -> Wire {
        let steps: Vec<(WireDirection, i32)> =
            input.split(",").map(|x| Wire::parse_step(x)).collect();
        let mut locs = HashSet::new();
        let mut loc_step = HashMap::new();
        let mut x = 0;
        let mut y = 0;
        let mut s = 1;
        for step in steps {
            let mut i = step.1;
            while i > 0 {
                match step.0 {
                    WireDirection::Right => x += 1,
                    WireDirection::Left => x -= 1,
                    WireDirection::Up => y += 1,
                    WireDirection::Down => y -= 1,
                }
                let loc = (x, y);
                locs.insert(loc);
                loc_step.insert(loc, s);
                i -= 1;
                s += 1;
            }
        }
        Wire { locs, loc_step }
    }

    pub fn length_at(&self, x: i32, y: i32) -> i32 {
        match self.loc_step.get(&(x, y)) {
            Some(i) => *i,
            None => panic!(),
        }
    }

    fn parse_step(input: &str) -> (WireDirection, i32) {
        let dir = match input.chars().next().unwrap() {
            'R' => WireDirection::Right,
            'L' => WireDirection::Left,
            'U' => WireDirection::Up,
            'D' => WireDirection::Down,
            _ => panic!(),
        };
        let len = &input[1..].parse::<i32>().unwrap();

        return (dir, *len);
    }
}

enum WireDirection {
    Up,
    Down,
    Left,
    Right,
}
