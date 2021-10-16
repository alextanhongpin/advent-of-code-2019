use num::rational::{BigRational, Ratio};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::f64::consts::PI;
use std::fs;
use std::iter::FromIterator;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();
    println!("part 1: {:?}", asteroid_count(&input));

    println!("part 2: {:?}", destroy_asteroid(&input, &(26, 29), 199));

    Ok(())
}

//https://www.reddit.com/r/adventofcode/comments/e8m1z3/2019_day_10_solutions/
// Since HashSet does not accept f64, this has to be returned as BigRational.
// In order to sort them from top, clockwise, the angle at 12 o'clock should be 0.
// For visualization, going up is -tive y and 0 degree, going down +tive y is 180degree.
// Going right is +tive x and 90 degree, going left is -tive x and 270 degree.
fn angle_to(a: &(i32, i32), b: &(i32, i32)) -> Option<BigRational> {
    let y: f64 = (b.1 - a.1).into();
    let x: f64 = (b.0 - a.0).into();

    let deg = -y.atan2(x) * 180_f64 / PI;
    let deg = if deg <= 90_f64 && deg >= 0_f64 {
        (deg - 90_f64).abs()
    } else if deg < 0_f64 {
        deg.abs() + 90_f64
    } else {
        450_f64 - deg
    };

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

fn destroy_asteroid(input: &str, pos: &(i32, i32), n: usize) -> (i32, i32) {
    let input = input.trim();
    let result = parse(input);

    // Find the angle of all other asteroids relative to monitoring station.
    let asteroids = result
        .iter()
        .filter(|&asteroid| asteroid != pos)
        .map(|asteroid2| (asteroid2.clone(), angle_to(&pos, asteroid2).unwrap()))
        .collect::<Vec<((i32, i32), BigRational)>>();

    // Group the asteroids by the given angle.
    let mut unique: HashMap<BigRational, (i32, i32)> = HashMap::new();

    for asteroid in asteroids.iter() {
        let angle = asteroid.1.clone();
        let curr = asteroid.0;
        let prev = unique.entry(angle).or_insert(curr);

        if curr == *prev {
            continue;
        }

        // Find manhattan distance to station.
        let curr_distance_to_station =
            (pos.1.abs() - curr.1.abs()).abs() + (pos.0.abs() - curr.0.abs()).abs();
        let prev_distance_to_station =
            (pos.1.abs() - prev.1.abs()).abs() + (pos.0.abs() - prev.0.abs()).abs();

        // Store the closest asteroid to station.
        if curr_distance_to_station < prev_distance_to_station {
            *prev = curr;
        }
    }

    // Sort by the angle.
    let mut asteroids: Vec<((i32, i32), BigRational)> = unique
        .keys()
        .map(|key| (*unique.get(key).unwrap(), key.clone()))
        .collect();

    asteroids.sort_by(|a, b| a.1.cmp(&b.1));

    asteroids[n].clone().0
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
        assert_eq!(8, asteroid_count(&input).1);

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
        assert_eq!(((5, 8), 33), asteroid_count(&input));

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
        assert_eq!(((1, 2), 35), asteroid_count(&input));

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
        assert_eq!(((6, 3), 41), asteroid_count(&input));

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
        assert_eq!(((11, 13), 210), asteroid_count(&input));
    }

    #[test]
    fn part2() {
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
        assert_eq!((11, 12), destroy_asteroid(&input, &(11, 13), 0));
        assert_eq!((12, 1), destroy_asteroid(&input, &(11, 13), 1));
        assert_eq!((12, 2), destroy_asteroid(&input, &(11, 13), 2));
    }

    #[test]
    fn test_angle() {
        assert_eq!(Ratio::from_float(180_f64), angle_to(&(0, 0), &(0, 1)));
        assert_eq!(Ratio::from_float(135_f64), angle_to(&(0, 0), &(1, 1)));
        assert_eq!(Ratio::from_float(90_f64), angle_to(&(0, 0), &(1, 0)));
        assert_eq!(Ratio::from_float(0_f64), angle_to(&(0, 0), &(0, -1)));
        assert_eq!(Ratio::from_float(315_f64), angle_to(&(0, 0), &(-1, -1)));
        assert_eq!(Ratio::from_float(270_f64), angle_to(&(0, 0), &(-1, 0)));

        assert_eq!(Ratio::from_float(0_f64), angle_to(&(11, 13), &(11, 12)));
        assert_eq!(Ratio::from_float(180_f64), angle_to(&(11, 13), &(11, 14)));
    }
}
