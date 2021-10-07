use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    let lines = input.split('\n').collect::<Vec<&str>>();
    let part1 = distance(lines[0], lines[1]);
    println!("part1: {}", part1);

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Path {
    Vertical(i32, (i32, i32)),
    Horizontal(i32, (i32, i32)),
}

impl Path {
    fn intersect(self, other: &Self) -> Option<(i32, i32)> {
        match (self, other) {
            (Path::Vertical(x, (y0, y1)), &Path::Horizontal(y, (x0, x1))) => {
                let (x0, x1) = if x0 > x1 { (x1, x0) } else { (x0, x1) };
                let (y0, y1) = if y0 > y1 { (y1, y0) } else { (y0, y1) };
                if (x0 + 1..x1).contains(&x) && (y0 + 1..y1).contains(&y) {
                    Some((x, y))
                } else {
                    None
                }
            }
            (Path::Horizontal(y, (x0, x1)), &Path::Vertical(x, (y0, y1))) => {
                let (x0, x1) = if x0 > x1 { (x1, x0) } else { (x0, x1) };
                let (y0, y1) = if y0 > y1 { (y1, y0) } else { (y0, y1) };
                if (x0 + 1..x1).contains(&x) && (y0 + 1..y1).contains(&y) {
                    Some((x, y))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

fn steps(wire1: &str, wire2: &str) -> i32 {
    let mut min: i32 = i32::MAX;

    let mut step1: i32 = 0;
    for w1 in parse(wire1).iter() {
        step1 += match w1 {
            &Path::Vertical(_, (y0, y1)) => (y1.abs() - y0.abs()).abs(),
            &Path::Horizontal(_, (x0, x1)) => (x1.abs() - x0.abs()).abs(),
        };
        let mut step2: i32 = 0;
        for w2 in parse(wire2).iter() {
            step2 += match w2 {
                &Path::Vertical(_, (y0, y1)) => (y1.abs() - y0.abs()).abs(),
                &Path::Horizontal(_, (x0, x1)) => (x1.abs() - x0.abs()).abs(),
            };

            if let Some((x, y)) = w1.intersect(w2) {
                // Corrected steps.
                let delta1 = match w1 {
                    &Path::Vertical(_, (_, y1)) => (y1 - y).abs(),
                    &Path::Horizontal(_, (_, x1)) => (x1 - x).abs(),
                };

                let delta2 = match w2 {
                    &Path::Vertical(_, (_, y1)) => (y1 - y).abs(),
                    &Path::Horizontal(_, (_, x1)) => (x1 - x).abs(),
                };
                if step1 - delta1 + step2 - delta2 < min {
                    min = step1 - delta1 + step2 - delta2;
                }
            }
        }
    }

    min
}

fn distance(wire1: &str, wire2: &str) -> i32 {
    let mut min: i32 = i32::MAX;

    for w1 in parse(wire1).iter() {
        for w2 in parse(wire2).iter() {
            if let Some((x, y)) = w1.intersect(w2) {
                if x.abs() + y.abs() < min {
                    min = x.abs() + y.abs();
                }
            }
        }
    }

    min
}

fn parse(directions: &str) -> Vec<Path> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    let mut paths: Vec<Path> = Vec::new();
    for step in directions.split(',') {
        let mut step = step.chars();
        match step.next() {
            Some('U') => {
                let steps = step.collect::<String>().parse::<i32>().unwrap();
                paths.push(Path::Vertical(x, (y, y + steps)));
                y += steps;
            }
            Some('D') => {
                let steps = step.collect::<String>().parse::<i32>().unwrap();
                paths.push(Path::Vertical(x, (y, y - steps)));
                y -= steps;
            }
            Some('R') => {
                let steps = step.collect::<String>().parse::<i32>().unwrap();
                paths.push(Path::Horizontal(y, (x, x + steps)));
                x += steps;
            }
            Some('L') => {
                let steps = step.collect::<String>().parse::<i32>().unwrap();
                paths.push(Path::Horizontal(y, (x, x - steps)));
                x -= steps;
            }
            _ => panic!("invalid"),
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(6, distance("R8,U5,L5,D3", "U7,R6,D4,L4"));
        assert_eq!(
            159,
            distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            )
        );
        assert_eq!(
            135,
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
