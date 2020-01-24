use intcode::IntCode;
use std::fmt;
use std::fs;
use std::mem::discriminant;

fn main() {
    let input = fs::read_to_string("aoc-13/input.txt").unwrap();
    let cab = ArcadeCabinet::from_string(&input);
    // cab.print();
    let block_tiles = cab.count_tiles(ArcadeCabinetTile::Block);
    println!("Block tiles: {}", block_tiles);
}

struct ArcadeCabinet {
    cpu: IntCode,
    output_index: usize,
    screen: [[ArcadeCabinetTile; 40]; 26],
}

impl ArcadeCabinet {
    pub fn from_string(input: &str) -> Self {
        let mut cpu = IntCode::from_string(input);
        cpu.execute().unwrap();
        let mut cab = ArcadeCabinet {
            cpu,
            output_index: 0,
            screen: [[ArcadeCabinetTile::Empty; 40]; 26],
        };
        cab.update_state();
        cab
    }

    pub fn print(&self) {
        let out = self.screen.iter().fold(String::from("\n"), |mut acc, row| {
            let r = row.iter().fold(String::from(" "), |mut acc, tile| {
                acc.push(char::from(*tile));
                acc
            });
            acc.push_str(&r);
            acc.push('\n');
            acc
        });
        println!("{}", out);
    }

    pub fn count_tiles(&self, tile: ArcadeCabinetTile) -> u32 {
        self.screen.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |acc, t| match tile.is_eq(t) {
                true => acc + 1,
                false => acc,
            })
        })
    }

    fn update_state(&mut self) {
        let output = self.cpu.get_output();
        while self.output_index < output.len() {
            let slc = &output[self.output_index..self.output_index + 3];
            if let [x, y, t] = slc {
                self.screen[*y as usize][*x as usize] = ArcadeCabinetTile::from(*t);
            } else {
                panic!(
                    "Input slice of incorrect size, expected len 3, got: {:?}",
                    slc
                )
            }
            self.output_index = self.output_index + 3;
        }
    }
}

#[derive(Copy, Clone)]
enum ArcadeCabinetTile {
    Empty,
    Wall,
    Block,
    PaddleHorizontal,
    Ball,
}

impl ArcadeCabinetTile {
    pub fn is_eq(&self, other: &ArcadeCabinetTile) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl From<isize> for ArcadeCabinetTile {
    fn from(i: isize) -> Self {
        match i {
            0 => ArcadeCabinetTile::Empty,
            1 => ArcadeCabinetTile::Wall,
            2 => ArcadeCabinetTile::Block,
            3 => ArcadeCabinetTile::PaddleHorizontal,
            4 => ArcadeCabinetTile::Ball,
            _ => panic!("No defined tile for {}", i),
        }
    }
}

impl From<ArcadeCabinetTile> for char {
    fn from(i: ArcadeCabinetTile) -> Self {
        match i {
            ArcadeCabinetTile::Empty => '0',
            ArcadeCabinetTile::Wall => '1',
            ArcadeCabinetTile::Block => '2',
            ArcadeCabinetTile::PaddleHorizontal => '3',
            ArcadeCabinetTile::Ball => '4',
        }
    }
}

impl fmt::Debug for ArcadeCabinetTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
