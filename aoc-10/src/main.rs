mod asteroid_field_point;
pub mod char_enum;
pub mod point_util;
pub mod space_map;

use asteroid_field_point::AsteroidFieldPoint;
use space_map::SpaceMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("aoc-10/input.txt").unwrap();
    let mut map = SpaceMap::<AsteroidFieldPoint>::from_string(&input);
    let max = map.max_visible();
    println!("Most visible (count, x, y): {:?}", max);
    let vapor_order = map.vaporize((max.1, max.2));
    println!("200th vaporized: {:?}", vapor_order[199]);
}
