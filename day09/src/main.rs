use intcode::Intcode;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();

    let mut program = Intcode::new(&input, vec![1]);
    program.run_until_halt();
    println!("part 1: {:?}", program.output);

    let mut program = Intcode::new(&input, vec![2]);
    program.run_until_halt();
    println!("part 1: {:?}", program.output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut program = Intcode::new(input.into(), vec![]);
        program.run_until_halt();
        assert_eq!(Intcode::parse(input), program.output, "test1");

        let input = "1102,34915192,34915192,7,4,7,99,0";
        let mut program = Intcode::new(input.into(), vec![]);
        program.run_until_halt();
        assert_eq!(vec![1219070632396864], program.output, "test2");

        let input = "104,1125899906842624,99";
        let mut program = Intcode::new(input.into(), vec![]);
        program.run_until_halt();
        assert_eq!(vec![1125899906842624], program.output, "test3");
    }
}
