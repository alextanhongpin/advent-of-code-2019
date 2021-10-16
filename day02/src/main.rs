use intcode::Intcode;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    assert_eq!("5110675", exec(&input.trim(), 12, 2));

    for noun in 0..=99 {
        for verb in 0..=99 {
            if "19690720" == exec(&input.trim(), noun, verb) {
                assert_eq!(4847, noun * 100 + verb);
                break;
            }
        }
    }

    Ok(())
}

fn exec(code: &str, noun: i128, verb: i128) -> String {
    let mut program = Intcode::from(code.to_string());
    program.codes[1] = noun;
    program.codes[2] = verb;

    program.run_until_halt();
    program.codes[0].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let mut program = Intcode::from("1,0,0,0,99".to_string());
        program.run_until_halt();
        assert_eq!(Intcode::parse("2,0,0,0,99"), program.codes, "test 1");

        let mut program = Intcode::from("2,3,0,3,99".to_string());
        program.run_until_halt();
        assert_eq!(Intcode::parse("2,3,0,6,99"), program.codes, "test 2");

        let mut program = Intcode::from("2,4,4,5,99,0".to_string());
        program.run_until_halt();
        assert_eq!(Intcode::parse("2,4,4,5,99,9801"), program.codes, "test 3");

        let mut program = Intcode::from("1,1,1,4,99,5,6,0,99".to_string());
        program.run_until_halt();
        assert_eq!(
            Intcode::parse("30,1,1,4,2,5,6,0,99"),
            program.codes,
            "test 4"
        );
    }
}
