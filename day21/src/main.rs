use intcode::Intcode;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("./input.txt").trim();

    // (!A OR !B OR !C) AND D
    let instructions = "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
"
    .chars()
    .map(|c| c as i128)
    .collect::<Vec<_>>();
    let mut program = Intcode::new(input, instructions);
    let output = program.run_until_halt();
    println!("{:?}", output); // 19358870
    let output = output
        .into_iter()
        .map(|c| ((c as u8) as char))
        .collect::<String>();
    print!("out: {}", output);

    // (!A OR !B OR !C) AND D AND (!E OR !H)
    let instructions = "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT E T
AND H T
OR E T
AND T J
RUN
"
    .chars()
    .map(|c| c as i128)
    .collect::<Vec<_>>();

    let mut program = Intcode::new(input, instructions);
    let output = program.run_until_halt();
    println!("{:?}", output); // 19358870
    let output = output
        .into_iter()
        .map(|c| ((c as u8) as char))
        .collect::<String>();
    print!("out: {}", output);
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
