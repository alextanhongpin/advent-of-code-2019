use intcode::Intcode;

use std::error::Error;
use std::fs;

const MAX: i128 = 50;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();

    let output = (0..MAX)
        .flat_map(|i| {
            (0..MAX)
                .flat_map(|j| {
                    let mut program = Intcode::new(&input, vec![i, j]);
                    program.run_until_halt()
                })
                .collect::<Vec<i128>>()
        })
        .sum::<i128>();
    assert_eq!(226, output);

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
