use intcode::Intcode;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();

    let mut program = Intcode::new(&input, vec![1]);
    let mut last: Option<i128> = None;
    loop {
        let result = program.exec();
        if program.done {
            break;
        }
        last = result;
    }
    assert_eq!(Some(15508323), last);
    assert_eq!(Some(9006327), Intcode::new(&input, vec![5]).exec());

    Ok(())
}
