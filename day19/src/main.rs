use intcode::Intcode;

use std::error::Error;
use std::fs;

const MAX: i128 = 50;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();

    assert_eq!(226, area(input, MAX));
    assert_eq!(7900946, part2(input));

    Ok(())
}

fn area(input: &str, rng: i128) -> i128 {
    (0..rng)
        .flat_map(|i| {
            (0..rng)
                .flat_map(|j| {
                    let mut program = Intcode::new(&input, vec![i, j]);
                    program.run_until_halt()
                })
                .collect::<Vec<i128>>()
        })
        .sum::<i128>()
}

fn check_area(input: &str, pos: (i128, i128), rng: i128) -> bool {
    let mut area = 0;

    for i in 0..rng {
        for j in 0..rng {
            let mut program = Intcode::new(&input, vec![pos.0 + i, pos.1 + j]);
            let output = program.run_until_halt();
            if output[0] != 1 {
                return false;
            }
            area += 1;
        }
    }
    area == rng * rng
}

fn part2(input: &str) -> i128 {
    let mut x = 100;
    let mut y = 100;

    let check_corners = |x, y| {
        let mut program = Intcode::new(&input, vec![x - 99, y + 99]);
        let output = program.run_until_halt();
        if output[0] == 1 && check_area(input, (x - 99, y), 100) {
            Some((x - 99, y))
        } else {
            None
        }
    };
    let mut result = 0;

    loop {
        let mut program = Intcode::new(&input, vec![x, y]);
        let output = program.run_until_halt();
        match output[0] {
            0 => {
                // As long as y is 0, keep moving down.
                y += 1;
                continue;
            }
            1 => {
                // Hits y.
                match check_corners(x, y) {
                    Some((x, y)) => {
                        result = x * 10_000 + y;
                        break;
                    }
                    None => x += 1,
                }
            }
            _ => panic!("Unexpected output"),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
