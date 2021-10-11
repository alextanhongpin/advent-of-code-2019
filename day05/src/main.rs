use intcode::program;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    assert_eq!(15508323, program(&input.trim(), 1));
    assert_eq!(9006327, program(&input.trim(), 5));

    Ok(())
}
