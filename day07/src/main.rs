use intcode::{program, Intcode};
use itertools::Itertools;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    println!("part 1: {}", amplify(&input.trim()));

    assert_eq!(35993240, feedback_loop(&input.trim()));

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
    let mut max: i32 = i32::MIN;

    for phases in (5..=9).permutations(5) {
        let mut out: i32 = 0;
        let mut programs: Vec<Intcode> = vec![
            Intcode::new(input, vec![phases[0], 0]),
            Intcode::new(input, vec![phases[1]]),
            Intcode::new(input, vec![phases[2]]),
            Intcode::new(input, vec![phases[3]]),
            Intcode::new(input, vec![phases[4]]),
        ];
        'inner: loop {
            for i in 0..5 {
                if programs[i].waiting {
                    continue;
                }
                let output = programs[i].exec();
                if programs[i].done {
                    break 'inner;
                }

                match output {
                    Some(n) => {
                        programs[(i + 1) % 5].set_input(n);
                        out = n;
                    }
                    None => continue,
                }
            }
        }
        if out > max {
            max = out;
        }
    }
    max
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

    #[test]
    fn part2() {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        assert_eq!(139629729, feedback_loop(input.into()));

        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        assert_eq!(18216, feedback_loop(input.into()));
    }
}
