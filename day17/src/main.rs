use intcode::Intcode;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let pixels = ascii(&input);

    let start = pixels
        .iter()
        .find_map(|(key, val)| if val == "^" { Some(key) } else { None })
        .unwrap();

    let mut pos = *start;

    use Direction::*;
    let mut direction = UP;

    // Keep track of the path travelled.
    let mut walked: HashMap<(usize, usize), usize> = HashMap::new();

    'walk: loop {
        let moves = match direction {
            UP => vec![UP, LEFT, RIGHT],
            DOWN => vec![DOWN, RIGHT, LEFT],
            LEFT => vec![LEFT, DOWN, UP],
            RIGHT => vec![RIGHT, UP, DOWN],
            NONE => break,
        };
        for mv in moves {
            let new_pos = match mv {
                UP if pos.1 > 0 => (pos.0, pos.1 - 1),
                DOWN => (pos.0, pos.1 + 1),
                LEFT if pos.0 > 0 => (pos.0 - 1, pos.1),
                RIGHT => (pos.0 + 1, pos.1),
                _ => continue,
            };

            if pixels.get(&new_pos) == Some(&"#".to_string()) {
                pos = new_pos;
                let cached = walked.entry(pos.clone()).or_insert(0);
                *cached += 1;
                direction = mv;
                continue 'walk;
            }
        }
        break;
    }

    let sum_of_alignment_parameters: usize = walked
        .into_iter()
        .map(|(pos, count)| if count > 1 { pos.0 * pos.1 } else { 0 })
        .sum();
    assert_eq!(5868, sum_of_alignment_parameters);

    Ok(())
}

fn ascii(input: &str) -> HashMap<(usize, usize), String> {
    let mut program = Intcode::from(input.trim().to_string());
    let pixels = program.run_until_halt();

    let mut result: HashMap<(usize, usize), String> = HashMap::new();
    let mut pos: (usize, usize) = (0, 0);
    for px in pixels {
        match px {
            35 => {
                result.insert(pos, "#".to_string());
            }
            46 => {
                result.insert(pos, ".".to_string());
            }
            10 => {
                pos.0 = 0;
                pos.1 += 1;
                continue;
            }
            94 => {
                result.insert(pos, "^".to_string());
            }
            i => unimplemented!("px: {}", i),
        }
        pos.0 += 1;
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
