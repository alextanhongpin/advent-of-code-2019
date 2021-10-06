use std::fs::File;
use std::io::prelude::*;
use std::io::{self};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("./src/input.txt")?;
    let f = io::BufReader::new(f);

    let mut part1 = 0;
    let mut part2 = 0;
    for line in f.lines() {
        let mass = line.unwrap().parse::<i32>()?;
        part1 += fuel_counter_upper(mass);
        part2 += fuel_counter_recursive(mass);
    }
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);

    Ok(())
}

fn fuel_counter_upper(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_counter_recursive(mass: i32) -> i32 {
    let fuel = fuel_counter_upper(mass);
    if fuel > 0 {
        fuel + fuel_counter_recursive(fuel)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(2, fuel_counter_upper(12));
        assert_eq!(2, fuel_counter_upper(14));
        assert_eq!(654, fuel_counter_upper(1969));
        assert_eq!(33583, fuel_counter_upper(100756));
    }

    #[test]
    fn part2() {
        assert_eq!(2, fuel_counter_recursive(12), "should return two");
        assert_eq!(966, fuel_counter_recursive(1969));
        assert_eq!(50346, fuel_counter_recursive(100756));
    }
}
