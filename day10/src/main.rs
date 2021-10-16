use num::rational::{Ratio, Rational32};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input = ".#..#
.....
#####
....#
...##";
    let input = input.trim();
    println!("part 1: {}", asteroid_count(&input));
}

fn asteroid_count(input: &str) -> usize {
    let mut count: usize = 0;

    let result = input.split('\n').collect::<Vec<&str>>();
    let result = result
        .iter()
        .enumerate()
        .flat_map(|(y, &row)| {
            let cols = row.chars().map(|c| c.to_string()).collect::<Vec<String>>();
            cols.into_iter()
                .enumerate()
                .map(move |(x, col)| (col.to_owned(), (x, y)))
                .filter(|(n, _)| n == "#")
                .map(|(_, p)| p)
        })
        .collect::<Vec<(usize, usize)>>();

    for asteroid1 in result.iter() {
        let asteroids: HashSet<String> = HashSet::from_iter(
            result
                .iter()
                .filter(|asteroid| asteroid != &asteroid1)
                .map(|asteroid2| gradient(&asteroid1, asteroid2)),
        );
        if asteroids.len() > count {
            count = asteroids.len();
        }
    }
    count
}

fn gradient(curr: &(usize, usize), input: &(usize, usize)) -> String {
    let num = input.1 as i32 - curr.1 as i32;
    let den = input.0 as i32 - curr.0 as i32;

    // Due to some weird logic, it will still attempt to divide by zero even when we use
    // Ratio::new_raw.
    // Casting to string avoids that.
    match (num, den) {
        (n, 0) => format!("x={}", n / n.abs()),
        (0, d) => format!("y={}", d / d.abs()),
        (n, d) => Rational32::new(n, d).to_string(),
    }
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
}
