use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    println!("{}", intcode(&input.trim()));

    Ok(())
}

fn intcode(code: &str) -> i32 {
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
                codes[p as usize] = 1;
                i += 2;
            }
            [4, p, ..] => {
                out = codes[p as usize];
                i += 2;
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
        assert_eq!(1, intcode("3,0,4,0,99"));
    }
}
