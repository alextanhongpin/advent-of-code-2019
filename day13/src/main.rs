use intcode::Intcode;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let mut program = Intcode::from(input);
    let output = program.run_until_halt();

    println!("output: len({:?}) {:?}", output.len(), output);

    let num_blocks = output
        .iter()
        .skip(2)
        .step_by(3)
        .filter(|&tile_id| tile_id == &2)
        .count();

    assert_eq!(207, num_blocks);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
