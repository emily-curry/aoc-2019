mod asteroid_field_point;
pub mod char_enum;
pub mod point_util;
pub mod space_map;

use asteroid_field_point::AsteroidFieldPoint;
use space_map::SpaceMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("aoc-10/input.txt").unwrap();
    let map = SpaceMap::<AsteroidFieldPoint>::from_string(&input);
    let max = map.max_visible();
    println!("{:?}", max);
}
