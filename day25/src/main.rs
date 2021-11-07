use intcode::{Intcode, Signal};
use std::io;
use std::io::prelude::*;

fn main() {
    let input = include_str!("./input.txt").trim();
    let mut program = Intcode::new(&input, vec![]);

    // Take the klein bottle, mutex, hypercube and mug at the security checkpoint.
    loop {
        match program.exec() {
            Signal::Waiting => {
                let output = program.output.clone();
                program.output.clear();
                let input = output
                    .into_iter()
                    .map(|n| (n as u8) as char)
                    .collect::<String>();
                print_single_line(&input);
                let input = read_line();
                let input = input.chars().map(|c| c as i128).collect::<Vec<i128>>();
                program.input.append(&mut input.clone());
            }
            Signal::Halt => {
                let output = program.output.clone();
                program.output.clear();
                let input = output
                    .into_iter()
                    .map(|n| (n as u8) as char)
                    .collect::<String>();
                print_single_line(&input);
                break;
            }
        }
    }
    program.run_until_halt();
}

fn print_single_line(input: &str) {
    print!("{}", input);

    // Flush to actually display the output.
    io::stdout().flush().unwrap();
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
