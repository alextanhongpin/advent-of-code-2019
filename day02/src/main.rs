use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    println!("part 1: {:?}", exec_intcode(&input.trim()));

    Ok(())
}

fn exec_intcode(code: &str) -> String {
    let mut codes: Vec<String> = code.split(',').map(str::to_string).collect();

    codes[1] = "12".into();
    codes[2] = "2".into();

    let solution = intcode(&codes.join(","));
    if let Some(&s) = solution.split(',').take(1).collect::<Vec<&str>>().first() {
        s.into()
    } else {
        "".into()
    }
}

fn intcode(code: &str) -> String {
    let mut codes: Vec<i32> = code
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut i = 0;
    loop {
        match codes[i..] {
            [1, a, b, p] | [1, a, b, p, ..] => {
                codes[p as usize] = codes[a as usize] + codes[b as usize];
                i += 4;
            }
            [2, a, b, p] | [2, a, b, p, ..] => {
                codes[p as usize] = codes[a as usize] * codes[b as usize];
                i += 4;
            }
            [99] | [99, ..] => break,
            _ => break,
        }
    }

    codes
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!("2,0,0,0,99", intcode("1,0,0,0,99"), "test 1");
        assert_eq!("2,3,0,6,99", intcode("2,3,0,3,99"), "test 2");
        assert_eq!("2,4,4,5,99,9801", intcode("2,4,4,5,99,0"), "test 3");
        assert_eq!(
            "30,1,1,4,2,5,6,0,99",
            intcode("1,1,1,4,99,5,6,0,99"),
            "test 4"
        );
    }
}
