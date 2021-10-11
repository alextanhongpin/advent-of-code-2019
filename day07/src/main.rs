use intcode::program;
use itertools::Itertools;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    println!("part 1: {}", amplify(&input.trim()));

    Ok(())
}

fn amplify(input: &str) -> i32 {
    let mut max: i32 = i32::MIN;

    for phases in (0..=4).permutations(5) {
        let mut i: i32 = 0;
        for phase in phases {
            i = program(input, vec![phase, i]);
        }
        if i > max {
            max = i;
        }
    }

    max
}

fn feedback_loop(input: &str) -> i32 {
    panic!("not implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        assert_eq!(43210, amplify(input.into()));

        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        assert_eq!(54321, amplify(input.into()));

        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        assert_eq!(65210, amplify(input.into()));
    }
}
