mod helper;
mod part1;
mod part2;
mod tile;

use part1::*;
use part2::*;

fn main() {
    let input = include_str!("./input.txt");
    assert_eq!(26840049, part1(input));
    assert_eq!(407, part2(input, 200));
}
