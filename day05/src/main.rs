use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    assert_eq!(15508323, intcode(&input.trim(), 1));
    assert_eq!(9006327, intcode(&input.trim(), 5));

    Ok(())
}

fn intcode(code: &str, input: i32) -> i32 {
    let mut codes: Vec<i32> = code
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut out = 0;
    let mut i = 0;
    loop {
        match codes[i..] {
            [c, a, b, p, ..] if c % 100 == 1 => {
                match c {
                    1 => {
                        codes[p as usize] = codes[a as usize] + codes[b as usize];
                    }
                    101 => {
                        codes[p as usize] = a + codes[b as usize];
                    }
                    1001 => {
                        codes[p as usize] = codes[a as usize] + b;
                    }
                    1101 => {
                        codes[p as usize] = a + b;
                    }
                    _ => panic!("invalid: {}", c),
                }
                i += 4;
            }
            [c, a, b, p, ..] if c % 100 == 2 => {
                match c {
                    2 => {
                        codes[p as usize] = codes[a as usize] * codes[b as usize];
                    }
                    102 => {
                        codes[p as usize] = a * codes[b as usize];
                    }
                    1002 => {
                        codes[p as usize] = codes[a as usize] * b;
                    }
                    1102 => {
                        codes[p as usize] = a * b;
                    }
                    _ => panic!("invalid: {}", c),
                }
                i += 4;
            }
            [3, p, ..] => {
                codes[p as usize] = input;
                i += 2;
            }
            [c, p, ..] if c % 100 == 4 => {
                out = match c {
                    4 => codes[p as usize],
                    _ => p,
                };
                i += 2;
            }
            [c, a, b, ..] if c % 100 == 5 => {
                let (a, b) = match c {
                    5 => (codes[a as usize], codes[b as usize]),
                    105 => (a, codes[b as usize]),
                    1005 => (codes[a as usize], b),
                    _ => (a, b),
                };
                i = if a != 0 { b as usize } else { i + 2 }
            }
            [c, a, b, ..] if c % 100 == 6 => {
                let (a, b) = match c {
                    6 => (codes[a as usize], codes[b as usize]),
                    106 => (a, codes[b as usize]),
                    1006 => (codes[a as usize], b),
                    _ => (a, b),
                };
                i = if a == 0 { b as usize } else { i + 2 }
            }
            [c, a, b, p, ..] if c % 100 == 7 => {
                let (a, b) = match c {
                    7 => (codes[a as usize], codes[b as usize]),
                    107 => (a, codes[b as usize]),
                    1007 => (codes[a as usize], b),
                    _ => (a, b),
                };
                codes[p as usize] = if a < b { 1 } else { 0 };
                i += 4;
            }
            [c, a, b, p, ..] if c % 100 == 8 => {
                let (a, b) = match c {
                    8 => (codes[a as usize], codes[b as usize]),
                    108 => (a, codes[b as usize]),
                    1008 => (codes[a as usize], b),
                    _ => (a, b),
                };
                codes[p as usize] = if a == b { 1 } else { 0 };
                i += 4;
            }
            [99] | [99, ..] => break,
            _ => {
                i += 1;
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(1, intcode("3,0,4,0,99", 1));
    }

    #[test]
    fn part2() {
        let input = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(1, intcode(&input, 8), "eq8");
        assert_eq!(0, intcode(&input, 0), "neq8");

        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(1, intcode(&input, 0), "lt8");
        assert_eq!(0, intcode(&input, 8), "eq8");

        let input = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(1, intcode(&input, 8), "eq8");
        assert_eq!(0, intcode(&input, 0), "neq8");

        let input = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(1, intcode(&input, 0), "lt8");
        assert_eq!(0, intcode(&input, 8), "eq8");

        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(0, intcode(&input, 0), "zero");
        assert_eq!(1, intcode(&input, 1), "non-zero");

        let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(0, intcode(&input, 0), "zero");
        assert_eq!(1, intcode(&input, 1), "non-zero");

        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(999, intcode(&input, 7), "lt8");
        assert_eq!(1000, intcode(&input, 8), "eq8");
        assert_eq!(1001, intcode(&input, 9), "gt8");
    }
}
