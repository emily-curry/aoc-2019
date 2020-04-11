use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::ErrorKind as CrosstermErrorKind;
use intcode::{IntCode, IntCodeResultKind};
use std::cmp::Ordering;
use std::fmt;
use std::fs;
use std::mem::discriminant;
use std::thread::sleep;
use std::time::Duration;

const INSERT_QUARTERS: bool = true;
const AUTO_PLAY: bool = true;
const FRAME_DURATION: u64 = 150;

fn main() {
    let input = fs::read_to_string("aoc-13/input.txt").unwrap();
    let mut data: Vec<isize> = input
        .split(",")
        .map(|x| x.parse::<isize>().unwrap())
        .collect();
    if INSERT_QUARTERS {
        data[0] = 2;
    }
    let mut cab = ArcadeCabinet::from_vec(data);
    cab.run(&AUTO_PLAY);
}

struct ArcadeCabinet {
    cpu: IntCode,
    output_index: usize,
    screen: [[ArcadeCabinetTile; 40]; 26],
    score: usize,
}

impl ArcadeCabinet {
    pub fn from_vec(input: Vec<isize>) -> Self {
        let mut cpu = IntCode::from_vec(input);
        cpu.input(&vec![0]);
        let cab = ArcadeCabinet {
            cpu,
            output_index: 0,
            screen: [[ArcadeCabinetTile::Empty; 40]; 26],
            score: 0,
        };
        cab
    }

    pub fn run(&mut self, autoplay: &bool) {
        while let IntCodeResultKind::Yield = self.cpu.execute().unwrap().kind {
            self.update_state();
            self.print();
            let input = match autoplay {
                false => self.get_user_input().unwrap(),
                true => self.get_auto_input(),
            };
            self.cpu.input(&vec![input])
        }
        self.update_state();
        self.print();
        println!(" ▬▬▬▬▬▬▬▬▬ Final Score ● {:0>5} ▬▬▬▬▬▬▬▬▬▬", self.score);
    }

    fn print(&self) {
        let tiles_left = self.count_tiles(ArcadeCabinetTile::Block);
        println!(
            " ██Left: {:0>3}███████████████Score: {:0>5}██",
            tiles_left, self.score
        );
        let out = self.screen.iter().fold(String::from(""), |mut acc, row| {
            let r = row.iter().fold(String::from(" "), |mut acc, tile| {
                acc.push(char::from(*tile));
                acc
            });
            acc.push_str(&r);
            acc.push('\n');
            acc
        });
        println!("{}", out);
        sleep(Duration::from_millis(FRAME_DURATION));
    }

    fn count_tiles(&self, tile: ArcadeCabinetTile) -> u32 {
        self.screen.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |acc, t| match tile == *t {
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
                if *x == -1 && *y == 0 {
                    self.score = *t as usize;
                } else {
                    self.screen[*y as usize][*x as usize] = ArcadeCabinetTile::from(*t);
                }
            } else {
                panic!(
                    "Input slice of incorrect size, expected len 3, got: {:?}",
                    slc
                )
            }
            self.output_index = self.output_index + 3;
        }
    }

    fn get_user_input(&self) -> Result<isize, CrosstermErrorKind> {
        loop {
            if poll(Duration::from_millis(500))? {
                match read()? {
                    Event::Key(event) => match event.code {
                        KeyCode::Left => return Ok(-1),
                        KeyCode::Right => return Ok(1),
                        KeyCode::Char(' ') => return Ok(0),
                        _ => continue,
                    },
                    _ => continue,
                }
            }
        }
    }

    fn get_auto_input(&self) -> isize {
        let ball_x = self
            .screen
            .iter()
            .find_map(|x| x.iter().position(|ix| *ix == ArcadeCabinetTile::Ball))
            .unwrap();
        let paddle_x = self
            .screen
            .iter()
            .find_map(|x| {
                x.iter()
                    .position(|ix| *ix == ArcadeCabinetTile::PaddleHorizontal)
            })
            .unwrap();
        match paddle_x.partial_cmp(&ball_x).unwrap() {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
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

impl PartialEq<ArcadeCabinetTile> for ArcadeCabinetTile {
    fn eq(&self, other: &ArcadeCabinetTile) -> bool {
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
            ArcadeCabinetTile::Empty => ' ',
            ArcadeCabinetTile::Wall => '█',
            ArcadeCabinetTile::Block => '■',
            ArcadeCabinetTile::PaddleHorizontal => '▬',
            ArcadeCabinetTile::Ball => '●',
        }
    }
}

impl fmt::Debug for ArcadeCabinetTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}
