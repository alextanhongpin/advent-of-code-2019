use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    let lines = input.split('\n').collect::<Vec<&str>>();
    if let Some(part1) = distance(lines[0], lines[1]) {
        println!("part1: {}", part1);
    }

    let part2 = steps(lines[0], lines[1]);
    println!("part2: {}", part2);

    Ok(())
}

fn steps(wire1: &str, wire2: &str) -> i32 {
    let wire1 = parse(wire1);
    let wire2 = parse(wire2);

    let p1: HashSet<(i32, i32)> = wire1.keys().cloned().collect();
    let p2: HashSet<(i32, i32)> = wire2.keys().cloned().collect();
    let intersection = p1.intersection(&p2);

    let mut min_steps: i32 = i32::MAX;
    for &intersect in intersection.into_iter() {
        if !wire1.contains_key(&intersect) {
            continue;
        }
        let step1 = wire1.get(&intersect).unwrap();
        let step2 = wire2.get(&intersect).unwrap();
        if step1 + step2 < min_steps {
            min_steps = step1 + step2;
        }
    }
    min_steps
}

fn distance(wire1: &str, wire2: &str) -> Option<i32> {
    let points1: HashSet<(i32, i32)> = parse(wire1).keys().cloned().collect();
    let points2: HashSet<(i32, i32)> = parse(wire2).keys().cloned().collect();
    let intersection = points1.intersection(&points2);
    intersection
        .into_iter()
        .map(|(a, b)| a.abs() + b.abs())
        .min()
}

fn parse(directions: &str) -> HashMap<(i32, i32), i32> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut points: HashMap<(i32, i32), i32> = HashMap::new();
    let mut idx: i32 = 1;

    for step in directions.split(',') {
        let mut step = step.chars();
        let dir = step.next();
        let steps = step.collect::<String>().parse::<i32>().unwrap();
        match dir {
            Some('U') => {
                for i in 1..=steps {
                    points.insert((x, y + i), idx);
                    idx += 1;
                }
                y += steps;
            }
            Some('D') => {
                for i in 1..=steps {
                    points.insert((x, y - i), idx);
                    idx += 1;
                }
                y -= steps;
            }
            Some('R') => {
                for i in 1..=steps {
                    points.insert((x + i, y), idx);
                    idx += 1;
                }
                x += steps;
            }
            Some('L') => {
                for i in 1..=steps {
                    points.insert((x - i, y), idx);
                    idx += 1;
                }
                x -= steps;
            }
            _ => panic!("invalid"),
        }
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(Some(6), distance("R8,U5,L5,D3", "U7,R6,D4,L4"));
        assert_eq!(
            Some(159),
            distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            )
        );
        assert_eq!(
            Some(135),
            distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        );
    }

    #[test]
    fn part2() {
        assert_eq!(30, steps("R8,U5,L5,D3", "U7,R6,D4,L4"), "test1");

        assert_eq!(
            610,
            steps(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            "test2",
        );
        assert_eq!(
            410,
            steps(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            "test3",
        );
    }
}
