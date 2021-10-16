use intcode::Intcode;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();

    let mut program = Intcode::from(input.to_string());
    program.set_input(1);
    program.run_until_halt();
    assert_eq!(Some(&15508323), program.output.last());

    let mut program = Intcode::from(input.to_string());
    program.set_input(5);
    program.run_until_halt();
    assert_eq!(Some(&9006327), program.output.last());

    Ok(())
}
