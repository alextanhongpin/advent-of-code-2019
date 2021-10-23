use intcode::{Intcode, Signal};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    let mut program = Intcode::from(input);
    let output = program.run_until_halt();

    let num_blocks = output
        .iter()
        .skip(2)
        .step_by(3)
        .filter(|&tile_id| tile_id == &2)
        .count();

    assert_eq!(207, num_blocks);

    // Part 2.
    let input = fs::read_to_string("./src/input.txt")?;
    let input = "2".to_owned() + &input[1..].to_owned();

    let mut program = Intcode::from(input);

    // Play the game.
    loop {
        match program.exec() {
            Signal::Halt => break,
            // Every wait is one frame refresh.
            // By then, the ball and paddle position would have changed.
            Signal::Waiting => {
                let output = program.output;
                let mut ball = (0, 0);
                let mut paddle = (0, 0);
                for chunk in output.chunks(3) {
                    match chunk {
                        &[x, y, tile_id] => match tile_id {
                            0 => {}               // Empty tile.
                            1 => {}               // Wall.
                            2 => {}               // Block.
                            3 => paddle = (x, y), // Horizontal paddle.
                            4 => ball = (x, y),   // Ball.
                            n => println!("score: {}", n),
                        },
                        _ => unimplemented!(),
                    }
                }
                program.output = vec![];
                // Input is left or right.
                // Paddle must catch up with the ball position.
                program.set_input(if paddle.0 > ball.0 {
                    // Tilt to the left
                    -1
                } else if paddle.0 < ball.0 {
                    // Tilt to the right.
                    1
                } else {
                    0
                });
                continue;
            }
        }
    }

    for chunk in program.output.chunks(3) {
        match chunk {
            &[_, _, tile_id] => match tile_id {
                0 | 1 | 2 | 3 | 4 => {}
                n => {
                    println!("score: {}", n);
                    assert_eq!(10247, n);
                }
            },
            _ => unimplemented!(),
        }
    }

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
