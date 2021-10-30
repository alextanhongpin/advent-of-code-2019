use intcode::Intcode;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::{fmt, fs};

mod grid;
use grid::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    UP(usize),
    DOWN(usize),
    LEFT(usize),
    RIGHT(usize),
}

impl Direction {
    fn steps(self) -> usize {
        use Direction::*;
        match self {
            UP(n) | DOWN(n) | LEFT(n) | RIGHT(n) => n,
        }
    }
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
    let mut program = Intcode::from(input.to_string());
    let output = program.run_until_halt();
    let pixels = ascii(output.clone());
    draw_map(output.clone());

    let start = pixels
        .iter()
        .find_map(|(key, val)| if val == "^" { Some(key) } else { None })
        .unwrap();

    let mut pos = *start;

    use Direction::*;
    let mut direction = UP(0);
    let mut prev_direction = direction;

    // Keep track of the path travelled.
    let mut steps: Vec<Direction> = vec![];

    'walk: loop {
        let moves = match direction {
            UP(n) => vec![UP(n + 1), LEFT(0), RIGHT(0)],
            DOWN(n) => vec![DOWN(n + 1), RIGHT(0), LEFT(0)],
            LEFT(n) => vec![LEFT(n + 1), DOWN(0), UP(0)],
            RIGHT(n) => vec![RIGHT(n + 1), UP(0), DOWN(0)],
        };
        // Check if the robot can move forward.

        for mv in moves {
            let new_pos = match mv {
                UP(_) if pos.1 > 0 => (pos.0, pos.1 - 1),
                DOWN(_) => (pos.0, pos.1 + 1),
                LEFT(_) if pos.0 > 0 => (pos.0 - 1, pos.1),
                RIGHT(_) => (pos.0 + 1, pos.1),
                _ => continue,
            };

            if pixels.get(&new_pos) == Some(&"#".to_string()) {
                // If the direction changes, just change direction without moving.
                if std::mem::discriminant(&direction) != std::mem::discriminant(&mv) {
                    //println!("moved: {:?}, direction: {:?}, mv: {:?}", pos, direction, mv);
                    let n = direction.steps();
                    if n != 0 {
                        steps.push(match (prev_direction, direction) {
                            (UP(_), RIGHT(_)) => RIGHT(n),
                            (UP(_), LEFT(_)) => LEFT(n),
                            (RIGHT(_), DOWN(_)) => RIGHT(n),
                            (RIGHT(_), UP(_)) => LEFT(n),
                            (DOWN(_), LEFT(_)) => RIGHT(n),
                            (DOWN(_), RIGHT(_)) => LEFT(n),
                            (LEFT(_), DOWN(_)) => LEFT(n),
                            (LEFT(_), UP(_)) => RIGHT(n),
                            _ => unimplemented!(),
                        });
                    }
                    //println!(
                    //"steps: {:?}\n",
                    //steps
                    //.iter()
                    //.map(ToString::to_string)
                    //.collect::<Vec<String>>()
                    //.join(",")
                    //);
                    prev_direction = direction;
                    direction = mv;
                    continue 'walk;
                }
                pos = new_pos;
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
            .join(",");
        let main_routine = to_ascii(&(path.clone() + "\n"));
        let function_a = to_routine(&mapping["A"]);
        let function_b = to_routine(&mapping["B"]);
        let function_c = to_routine(&(mapping["C"]));
        let camera_feed = to_ascii("n\n");

        let mut input: Vec<i128> = vec![];
        input.extend(
            main_routine
                .iter()
                .map(|&n| n as i128)
                .collect::<Vec<i128>>(),
        );
        input.extend(function_a);
        input.extend(function_b);
        input.extend(function_c);
        input.extend(
            camera_feed
                .iter()
                .map(|&n| n as i128)
                .collect::<Vec<i128>>(),
        );

        let mut program = Intcode::new(&start.trim(), input);
        let output = program.run_until_halt();
        //draw_map(output.clone());
        println!(
            "a: {:?}, b: {:?}, c: {:?}",
            mapping["A"], mapping["B"], mapping["C"]
        );
        println!(
            "input: {:?}, output: {:?}, x: {:?}\n",
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

fn to_routine(input: &str) -> Vec<i128> {
    let re = Regex::new(r"([R|L])(\d+)").unwrap();
    let mut out = re
        .captures_iter(input)
        .flat_map(|cap| {
            vec![
                cap[1].to_owned(),
                ",".to_string(),
                cap[2].to_owned(),
                ",".to_string(),
            ]
        })
        .flat_map(|s| s.chars().collect::<Vec<char>>())
        .map(|c| c as i128)
        .collect::<Vec<i128>>();
    out.pop();
    out.push('\n' as i128);
    out
}

fn sum_of_alignment_parameters(input: &str) -> usize {
    let mut program = Intcode::from(input.to_string());
    let output = program.run_until_halt();
    let pixels = ascii(output);

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
            if i + j > n {
                continue;
            }
            let a = &steps[n - i - j..n - i]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join("");

            let mut dedup: HashMap<String, String> = HashMap::new();
            dedup.insert(c.to_string(), "C".to_string());

            let is_a: bool = path.starts_with(a);
            if is_a {
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
                if !is_a {
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
            if dedup.len() == 5 {
                for (k, v) in dedup.iter() {
                    println!("k: {:?} {:?}", k, v);
                }
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
        assert_eq!(vec![76, 44, 54, 44, 76, 44, 50, 10, 1], result);
    }
}
