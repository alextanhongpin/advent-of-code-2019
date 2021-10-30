use intcode::Intcode;
use std::collections::HashMap;
use std::error::Error;
use std::{fmt, fs};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    UP(usize),
    DOWN(usize),
    LEFT(usize),
    RIGHT(usize),
}

// Prints UP(10) to U10.
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Direction::*;
        match self {
            UP(n) => write!(f, "U{:?}", n),
            DOWN(n) => write!(f, "D{:?}", n),
            LEFT(n) => write!(f, "L{:?}", n),
            RIGHT(n) => write!(f, "R{:?}", n),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    assert_eq!(5680, sum_of_alignment_parameters(&input));

    println!("running part 2");
    part2(&input);

    Ok(())
}

fn part2(input: &str) -> i32 {
    let pixels = ascii(&input);

    let start = pixels
        .iter()
        .find_map(|(key, val)| if val == "^" { Some(key) } else { None })
        .unwrap();

    let mut pos = *start;

    use Direction::*;
    let mut direction = UP(0);

    // Keep track of the path travelled.
    let mut steps: Vec<Direction> = vec![];

    'walk: loop {
        let moves = match direction {
            UP(n) => vec![UP(n + 1), LEFT(0), RIGHT(0)],
            DOWN(n) => vec![DOWN(n + 1), RIGHT(0), LEFT(0)],
            LEFT(n) => vec![LEFT(n + 1), DOWN(0), UP(0)],
            RIGHT(n) => vec![RIGHT(n + 1), UP(0), DOWN(0)],
        };

        for mv in moves {
            let new_pos = match mv {
                UP(_) if pos.1 > 0 => (pos.0, pos.1 - 1),
                DOWN(_) => (pos.0, pos.1 + 1),
                LEFT(_) if pos.0 > 0 => (pos.0 - 1, pos.1),
                RIGHT(_) => (pos.0 + 1, pos.1),
                _ => continue,
            };

            if pixels.get(&new_pos) == Some(&"#".to_string()) {
                pos = new_pos;
                // Compare enum by variant, not values.
                if std::mem::discriminant(&direction) != std::mem::discriminant(&mv) {
                    match (direction, mv) {
                        (UP(n), RIGHT(_)) => {
                            if n > 0 {
                                steps.push(RIGHT(n))
                            }
                        }
                        (UP(n), LEFT(_)) => steps.push(LEFT(n)),
                        (RIGHT(n), DOWN(_)) => steps.push(RIGHT(n)),
                        (RIGHT(n), UP(_)) => steps.push(LEFT(n)),
                        (DOWN(n), LEFT(_)) => steps.push(RIGHT(n)),
                        (DOWN(n), RIGHT(_)) => steps.push(LEFT(n)),
                        (LEFT(n), DOWN(_)) => steps.push(LEFT(n)),
                        (LEFT(n), UP(_)) => steps.push(RIGHT(n)),
                        _ => unimplemented!(),
                    }
                };
                direction = mv;
                continue 'walk;
            }
        }
        steps.push(direction);
        break;
    }

    let start = "2".to_string() + &input[1..];
    println!(
        "steps: {:?}",
        steps
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("")
    );
    let result = compress_path(steps);
    for (mapping, path) in result.iter() {
        let path = path
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",")
            + "\n";
        let main_routine = to_ascii(&path);

        let function_a = to_ascii(&(mapping["A"].to_owned() + "\n"));
        let function_b = to_ascii(&(mapping["B"].to_owned() + "\n"));
        let function_c = to_ascii(&(mapping["C"].to_owned() + "\n"));
        let camera_feed = to_ascii("y\n");
        let mut input: Vec<usize> = main_routine;
        input.extend(function_a);
        input.extend(function_b);
        input.extend(function_c);
        input.extend(camera_feed);

        let mut program = Intcode::new(&start.trim(), input.iter().map(|&n| n as i128).collect());
        let output = program.run_until_halt();
        println!(
            "a: {:?}, b: {:?}, c: {:?}",
            mapping["A"], mapping["B"], mapping["C"]
        );
        println!(
            "input: {:?}, output: {:?}, x: {:?}",
            path,
            output.iter().max(),
            output.iter().find(|&n| n == &('X' as i128))
        );
    }

    0
}

fn to_ascii(input: &str) -> Vec<usize> {
    input.chars().map(|c| c as usize).collect::<Vec<usize>>()
}

fn sum_of_alignment_parameters(input: &str) -> usize {
    let pixels = ascii(input);

    pixels
        .clone()
        .into_iter()
        .filter(|(pos, val)| {
            val == "#"
                && (pos.1 > 0 && pixels.get(&(pos.0, pos.1 - 1)) == Some(&"#".to_string()))
                && (pos.0 > 0 && pixels.get(&(pos.0 - 1, pos.1)) == Some(&"#".to_string()))
                && pixels.get(&(pos.0, pos.1 + 1)) == Some(&"#".to_string())
                && pixels.get(&(pos.0 + 1, pos.1)) == Some(&"#".to_string())
        })
        .map(|(pos, _)| pos.0 * pos.1)
        .sum::<usize>()
}

fn ascii(input: &str) -> HashMap<(usize, usize), String> {
    let mut program = Intcode::from(input.trim().to_string());
    let pixels = program.run_until_halt();

    let mut result: HashMap<(usize, usize), String> = HashMap::new();
    let mut pos: (usize, usize) = (0, 0);
    for px in pixels {
        match px {
            35 => {
                result.insert(pos, "#".to_string());
            }
            46 => {
                result.insert(pos, ".".to_string());
            }
            10 => {
                pos.0 = 0;
                pos.1 += 1;
                continue;
            }
            94 => {
                result.insert(pos, "^".to_string());
            }
            i => unimplemented!("px: {}", i),
        }
        pos.0 += 1;
    }

    result
}

fn compress_path(steps: Vec<Direction>) -> Vec<(HashMap<String, String>, String)> {
    let mut result: Vec<(HashMap<String, String>, String)> = vec![];

    let path = steps
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("");

    let n = steps.len();
    for i in 1..=10 {
        if i > n {
            continue;
        }
        let c = &steps[n - i..]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("");

        let mut abs = path.split(c).filter(|s| s.len() > 0).collect::<Vec<&str>>();
        abs.sort();

        for j in 1..=10 {
            let a = &steps[n - i - j..n - i]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join("");

            let mut dedup: HashMap<String, String> = HashMap::new();
            dedup.insert(c.to_string(), "C".to_string());

            if path.starts_with(a) {
                dedup.insert(a.to_string(), "A".to_string());
            } else {
                dedup.insert(a.to_string(), "B".to_string());
            }

            for ab in abs.iter() {
                let mut sub = ab.split(a).filter(|s| s.len() > 0).collect::<Vec<&str>>();
                sub.sort();
                sub.dedup();
                if sub.len() != 1 {
                    continue;
                };
                let b = sub[0];
                if path.starts_with(b) {
                    dedup.insert(b.to_string(), "A".to_string());
                } else {
                    dedup.insert(b.to_string(), "B".to_string());
                }
            }
            if dedup.len() != 3 {
                continue;
            }

            let mut path = path.clone();
            for (k, v) in dedup.iter() {
                path = path.replace(&k.clone(), &v.clone());
            }

            for (k, v) in dedup.clone() {
                dedup.insert(v, k);
            }
            if path.len() > 10 {
                continue;
            }

            result.push((dedup, path));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = to_ascii("A,B,C,B,A,C\n");
        assert_eq!(vec![65, 44, 66, 44, 67, 44, 66, 44, 65, 44, 67, 10], result);

        let result = to_ascii("R,8,R,8\n");
        assert_eq!(vec![82, 44, 56, 44, 82, 44, 56, 10], result);

        let result = to_ascii("R,4,R,4,R,8\n");
        assert_eq!(vec![82, 44, 52, 44, 82, 44, 52, 44, 82, 44, 56, 10], result);

        let result = to_ascii("L,6,L,2\n");
        assert_eq!(vec![76, 44, 54, 44, 76, 44, 50, 10], result);
    }
}
