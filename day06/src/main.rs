use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    println!("part 1: {:?}", parse(&input.trim()));
    println!("part 2: {:?}", transport(&input.trim()));

    Ok(())
}

fn reverse_lookup(input: &str, from: &str, to: &str) -> Option<HashSet<String>> {
    let lines = input.split('\n');
    let mut points: HashSet<String> = HashSet::new();
    points.insert(to.into());

    let mut to = to;
    loop {
        for line in lines.clone() {
            let mut line = line.split(')');
            let line = (line.next()?, line.next()?);
            if line.1 == to {
                points.insert(line.0.into());
                to = line.0;
            }
        }
        if to == from {
            break;
        }
    }
    Some(points)
}

fn transport(input: &str) -> Option<usize> {
    let you: HashSet<String> = reverse_lookup(input, "COM".into(), "YOU".into())?;
    let san: HashSet<String> = reverse_lookup(input, "COM".into(), "SAN".into())?;

    Some(
        // Find the ones that are in YOU or in SAN but not both.
        you.symmetric_difference(&san)
            .cloned()
            .collect::<Vec<String>>()
            .len()
            - 2, // Remove YOU and SAN.
    )
}

fn parse(input: &str) -> Option<i32> {
    let lines = input.split('\n');

    let mut result: i32 = 0;
    let mut depth: i32 = 0;
    let mut counter: HashMap<&str, i32> = HashMap::new();
    counter.insert("COM", depth);
    depth += 1;

    loop {
        let matches: Vec<&str> = counter.keys().cloned().collect();
        let total: i32 = counter.values().sum();
        result += total;
        counter.clear();

        for line in lines.clone() {
            let mut line = line.split(')');
            let line = (line.next()?, line.next()?);
            if matches.contains(&line.0) {
                counter.insert(line.1, depth);
            }
        }

        if counter.is_empty() {
            break;
        }
        depth += 1;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

        assert_eq!(Some(42), parse(input));
    }

    #[test]
    fn part2() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        assert_eq!(Some(4), transport(&input));
    }
}
