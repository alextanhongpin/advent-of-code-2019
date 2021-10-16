use num::bigint::BigInt;
use num::rational::{BigRational, Ratio};
use std::collections::HashSet;
use std::error::Error;
use std::f64::consts::PI;
use std::fs;
use std::iter::FromIterator;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();
    println!("part 1: {:?}", asteroid_count(&input));

    // WIP
    //println!("part 2: {:?}", destroy_asteroid(&input, &(26, 29)));

    Ok(())
}

// Calculates the angle relative to 12 o'clock as 0 degree, 3 o'clock as 90 degree etc.
// Since HashSet does not accept f64, this has to be returned as BigRational.
fn angle_to(a: &(i32, i32), b: &(i32, i32)) -> Option<BigRational> {
    let y: f64 = (b.1 - a.1).into();
    let x: f64 = (b.0 - a.0).into();
    let deg = x.atan2(y) / PI * 180_f64;
    let deg = if deg < 0_f64 { deg + 360_f64 } else { deg };
    Ratio::from_float(deg)
}

fn parse(input: &str) -> Vec<(i32, i32)> {
    let result = input.split('\n').collect::<Vec<&str>>();
    result
        .iter()
        .enumerate()
        .flat_map(|(y, &row)| {
            let cols = row.chars().map(|c| c.to_string()).collect::<Vec<String>>();
            cols.into_iter()
                .enumerate()
                .map(move |(x, col)| (col.to_owned(), (x as i32, y as i32)))
                .filter(|(n, _)| n == "#")
                .map(|(_, p)| p)
        })
        .collect::<Vec<(i32, i32)>>()
}

fn destroy_asteroid(input: &str, pos: &(i32, i32)) -> Ratio<BigInt> {
    let input = input.trim();
    let result = parse(input);

    let mut asteroids = result
        .iter()
        .filter(|&asteroid| asteroid != pos)
        .map(|asteroid2| angle_to(&pos, asteroid2).unwrap())
        .collect::<Vec<BigRational>>();

    asteroids.sort();

    asteroids[199].clone()
}

fn asteroid_count(input: &str) -> ((i32, i32), usize) {
    let input = input.trim();
    let mut pos: (i32, i32) = (0, 0);
    let mut count: usize = 0;

    let result = parse(input);
    for asteroid1 in result.iter() {
        let asteroids: HashSet<BigRational> = HashSet::from_iter(
            result
                .iter()
                .filter(|asteroid| asteroid != &asteroid1)
                .map(|asteroid2| angle_to(asteroid1, asteroid2).unwrap()),
        );
        if asteroids.len() > count {
            pos = *asteroid1;
            count = asteroids.len();
        }
    }
    (pos, count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = ".#..#
.....
#####
....#
...##";
        assert_eq!(8, asteroid_count(&input));

        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        assert_eq!(33, asteroid_count(&input));

        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        assert_eq!(35, asteroid_count(&input));

        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        assert_eq!(41, asteroid_count(&input));

        let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        assert_eq!(210, asteroid_count(&input));
    }

    #[test]
    fn test_angle() {
        assert_eq!(Ratio::from_float(0_f64), angle_to(&(0, 0), &(0, 1)));
        assert_eq!(Ratio::from_float(45_f64), angle_to(&(0, 0), &(1, 1)));
        assert_eq!(Ratio::from_float(90_f64), angle_to(&(0, 0), &(1, 0)));
        assert_eq!(Ratio::from_float(180_f64), angle_to(&(0, 0), &(0, -1)));
        assert_eq!(Ratio::from_float(225_f64), angle_to(&(0, 0), &(-1, -1)));
        assert_eq!(Ratio::from_float(270_f64), angle_to(&(0, 0), &(-1, 0)));
    }
}
