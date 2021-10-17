use intcode::{Intcode, Signal};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
}

impl Direction {
    fn turn(self, left_or_right: i32) -> Self {
        let next = match left_or_right {
            0 => self as i32 - 1,
            1 => self as i32 + 1,
            _ => unimplemented!(),
        };
        Direction::from((next + 4) % 4)
    }
}

impl From<i32> for Direction {
    fn from(n: i32) -> Self {
        use Direction::*;
        match n {
            0 => UP,
            1 => RIGHT,
            2 => DOWN,
            3 => LEFT,
            _ => UP,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();

    let mut pos = (0, 0);
    let mut dir = Direction::UP;
    let mut panels: HashMap<(i32, i32), usize> = HashMap::new();
    panels.insert(pos, 0);

    // Part 1: with input 0.
    //let mut program = Intcode::new(&input, vec![*panels.get(&pos).unwrap() as i128]);
    //
    // Part 2: with input 1.
    let mut program = Intcode::new(&input, vec![1]);

    loop {
        match program.exec() {
            Signal::Waiting => {
                let left_or_right = program.output.pop().unwrap();
                let color = program.output.pop().unwrap();

                // Paint the panel.
                panels.insert(pos, color as usize);

                use Direction::*;
                dir = dir.turn(left_or_right as i32);
                pos = match dir {
                    UP => (pos.0, pos.1 - 1),
                    DOWN => (pos.0, pos.1 + 1),
                    LEFT => (pos.0 - 1, pos.1),
                    RIGHT => (pos.0 + 1, pos.1),
                };

                // Infer input on current block it steps on.
                let input = panels.get(&pos).unwrap_or(&0);
                program.set_input(input.to_owned() as i128);
            }
            Signal::Halt => {
                println!("program halt: {:?}", program.output);
                break;
            }
        }
    }
    println!("{}", panels.len());
    let max_x = panels.keys().cloned().map(|pos| pos.0).max().unwrap() + 1;
    let max_y = panels.keys().cloned().map(|pos| pos.1).max().unwrap() + 1;
    let mut vec: Vec<Vec<String>> = vec![vec![" ".to_string(); max_x as usize]; max_y as usize];
    for ((x, y), v) in panels {
        vec[y as usize][x as usize] = match v {
            0 => " ".to_string(),
            1 => "0".to_string(),
            _ => unimplemented!(),
        };
    }
    for row in vec.iter() {
        println!("{:?}", row.join(""));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::*;

    #[test]
    fn turn_right() {
        assert_eq!(RIGHT, UP.turn(1));
        assert_eq!(DOWN, RIGHT.turn(1));
        assert_eq!(LEFT, DOWN.turn(1));
        assert_eq!(UP, LEFT.turn(1));
    }

    #[test]
    fn turn_left() {
        assert_eq!(LEFT, UP.turn(0));
        assert_eq!(DOWN, LEFT.turn(0));
        assert_eq!(RIGHT, DOWN.turn(0));
        assert_eq!(UP, RIGHT.turn(0));
    }
}
