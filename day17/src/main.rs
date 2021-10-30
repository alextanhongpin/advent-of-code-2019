use intcode::Intcode;
use std::error::Error;
use std::fs;

mod direction;
mod grid;
use direction::*;
use grid::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    assert_eq!(5680, sum_of_alignment_parameters(&input));
    assert_eq!(895965, space_dusts(&input));

    Ok(())
}

fn space_dusts(input: &str) -> i128 {
    let mut program = Intcode::from(input.to_string());
    let output = program.run_until_halt();
    let pixels = ascii(output.clone());

    let start = pixels
        .iter()
        .find_map(|(key, val)| if val == "^" { Some(key) } else { None })
        .unwrap();

    let mut pos = *start;

    use Direction::*;
    let mut direction = UP(0);
    let mut prev_direction = direction;

    // Keep track of the path travelled.
    let mut steps: Vec<Direction> = vec![];

    'walk: loop {
        let moves = match direction {
            UP(n) => vec![UP(n + 1), LEFT(0), RIGHT(0)],
            DOWN(n) => vec![DOWN(n + 1), RIGHT(0), LEFT(0)],
            LEFT(n) => vec![LEFT(n + 1), DOWN(0), UP(0)],
            RIGHT(n) => vec![RIGHT(n + 1), UP(0), DOWN(0)],
        };

        for mv in moves {
            let new_pos = match mv {
                UP(_) if pos.1 > 0 => (pos.0, pos.1 - 1),
                DOWN(_) => (pos.0, pos.1 + 1),
                LEFT(_) if pos.0 > 0 => (pos.0 - 1, pos.1),
                RIGHT(_) => (pos.0 + 1, pos.1),
                _ => continue,
            };

            if pixels.get(&new_pos) == Some(&"#".to_string()) {
                if std::mem::discriminant(&direction) != std::mem::discriminant(&mv) {
                    let n = direction.steps();
                    if n != 0 {
                        steps.push(match (prev_direction, direction) {
                            (UP(_), RIGHT(_)) => RIGHT(n),
                            (UP(_), LEFT(_)) => LEFT(n),
                            (RIGHT(_), DOWN(_)) => RIGHT(n),
                            (RIGHT(_), UP(_)) => LEFT(n),
                            (DOWN(_), LEFT(_)) => RIGHT(n),
                            (DOWN(_), RIGHT(_)) => LEFT(n),
                            (LEFT(_), DOWN(_)) => LEFT(n),
                            (LEFT(_), UP(_)) => RIGHT(n),
                            _ => unimplemented!(),
                        });
                    }
                    prev_direction = direction;
                    direction = mv;
                    continue 'walk;
                }
                pos = new_pos;
                direction = mv;
                continue 'walk;
            }
        }
        steps.push(direction);
        break;
    }

    let start = "2".to_string() + &input[1..];
    let result = compress_path(steps);
    for (mapping, path) in result.iter() {
        let path = path
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let main_routine = to_ascii(&(path.clone() + "\n"));
        let function_a = to_routine(&mapping["A"]);
        let function_b = to_routine(&mapping["B"]);
        let function_c = to_routine(&(mapping["C"]));
        let camera_feed = to_ascii("n\n");

        if vec![&function_a, &function_b, &function_c]
            .iter()
            .any(|v| v.len() > 20)
        {
            continue;
        }

        let mut input: Vec<i128> = vec![];
        input.extend(main_routine);
        input.extend(function_a);
        input.extend(function_b);
        input.extend(function_c);
        input.extend(camera_feed);

        let mut program = Intcode::new(&start.trim(), input);
        let output = program.run_until_halt();
        draw_map(output.clone());

        if output.iter().find(|&n| {
            n == &(match direction {
                RIGHT(_) => '>',
                LEFT(_) => '<',
                UP(_) => '^',
                DOWN(_) => 'v',
            } as i128)
        }) != None
        {
            println!("path: {}", path);
            println!("a: {}", mapping["A"]);
            println!("b: {}", mapping["B"]);
            println!("c: {}", mapping["C"]);
            return output.into_iter().max().unwrap();
        }
    }

    0
}

fn sum_of_alignment_parameters(input: &str) -> usize {
    let mut program = Intcode::from(input.to_string());
    let output = program.run_until_halt();
    let pixels = ascii(output);

    pixels
        .clone()
        .into_iter()
        .filter(|(pos, val)| {
            val == "#"
                && (pos.1 > 0 && pixels.get(&(pos.0, pos.1 - 1)) == Some(&"#".to_string()))
                && (pos.0 > 0 && pixels.get(&(pos.0 - 1, pos.1)) == Some(&"#".to_string()))
                && pixels.get(&(pos.0, pos.1 + 1)) == Some(&"#".to_string())
                && pixels.get(&(pos.0 + 1, pos.1)) == Some(&"#".to_string())
        })
        .map(|(pos, _)| pos.0 * pos.1)
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = to_ascii("A,B,C,B,A,C\n");
        assert_eq!(vec![65, 44, 66, 44, 67, 44, 66, 44, 65, 44, 67, 10], result);

        let result = to_ascii("R,8,R,8\n");
        assert_eq!(vec![82, 44, 56, 44, 82, 44, 56, 10], result);

        let result = to_ascii("R,4,R,4,R,8\n");
        assert_eq!(vec![82, 44, 52, 44, 82, 44, 52, 44, 82, 44, 56, 10], result);

        let result = to_ascii("L,6,L,2\n");
        assert_eq!(vec![76, 44, 54, 44, 76, 44, 50, 10, 1], result);
    }
}
