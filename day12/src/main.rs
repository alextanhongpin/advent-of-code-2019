use num::Integer;
use num_bigint::BigInt;
use regex::Regex;
use std::cmp::Ordering;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    assert_eq!(11384, total_energy(&input, 1000));
    assert_eq!(
        BigInt::parse_bytes(b"452582583272768", 10).unwrap(),
        simulate(&input)
    );

    Ok(())
}

fn simulate(input: &str) -> BigInt {
    let mut step: i32 = 0;
    let mut moons = parse(input);
    let mut steps = (0, 0, 0);

    loop {
        let initial = moons.clone();
        let others = moons.clone();
        for (i, moon) in moons.iter_mut().enumerate() {
            let new_velocity = others
                .iter()
                .enumerate()
                .filter(|&(j, _)| i != j)
                .map(|(_, &other)| moon.cmp(other))
                .fold(moon.velocity, |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2));
            moon.update_velocity(new_velocity);
        }

        for moon in moons.iter_mut() {
            moon.update_position();
        }
        step += 1;

        if equal_position(initial.clone(), moons.clone(), 'x') && steps.0 == 0 {
            steps.0 = step * 2;
        }

        if equal_position(initial.clone(), moons.clone(), 'y') && steps.1 == 0 {
            steps.1 = step * 2;
        }

        if equal_position(initial.clone(), moons.clone(), 'z') && steps.2 == 0 {
            steps.2 = step * 2;
        }

        if steps.0 > 0 && steps.1 > 0 && steps.2 > 0 {
            break;
        }
    }

    BigInt::from(steps.0)
        .lcm(&BigInt::from(steps.1))
        .lcm(&BigInt::from(steps.2))
}

fn equal_position(initial: Vec<Moon>, current: Vec<Moon>, field: char) -> bool {
    let pos = match field {
        'x' => |moon: &Moon| moon.position.0,
        'y' => |moon: &Moon| moon.position.1,
        'z' => |moon: &Moon| moon.position.2,
        _ => unimplemented!(),
    };
    let left = initial.iter().map(pos).collect::<Vec<i32>>();
    let right = current.iter().map(pos).collect::<Vec<i32>>();
    left == right
}

fn total_energy(input: &str, steps: i32) -> i32 {
    let mut moons = parse(input);
    for _ in 0..steps {
        let others = moons.clone();
        for (i, moon) in moons.iter_mut().enumerate() {
            let new_velocity = others
                .iter()
                .enumerate()
                .filter(|&(j, _)| i != j)
                .map(|(_, &other)| moon.cmp(other))
                .fold(moon.velocity, |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2));
            moon.update_velocity(new_velocity);
        }

        for moon in moons.iter_mut() {
            moon.update_position();
        }
    }

    let total_energy = moons
        .iter()
        .map(|moon| moon.total_energy())
        .fold(0, |a, b| a + b);
    total_energy
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Moon {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
}

fn parse_row(input: &str) -> Moon {
    let re = Regex::new(r"<x=(-?\d+),\s*y=(-?\d+),\s*z=(-?\d+)\s*>").unwrap();
    let cap = re.captures(input).unwrap();
    Moon::new((
        cap[1].parse::<i32>().unwrap(),
        cap[2].parse::<i32>().unwrap(),
        cap[3].parse::<i32>().unwrap(),
    ))
}

fn parse(input: &str) -> Vec<Moon> {
    let input = input.trim();
    input
        .split('\n')
        .map(|row| parse_row(row))
        .collect::<Vec<Moon>>()
}

impl Moon {
    fn new(position: (i32, i32, i32)) -> Self {
        Moon {
            position: position,
            velocity: (0, 0, 0),
        }
    }

    fn cmp_pos(self, field: char, other: Moon) -> i32 {
        let (left, right) = match field {
            'x' => (self.position.0, other.position.0),
            'y' => (self.position.1, other.position.1),
            'z' => (self.position.2, other.position.2),
            _ => unimplemented!(),
        };
        match left.cmp(&right) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        }
    }

    fn cmp(self, other: Moon) -> (i32, i32, i32) {
        (
            self.cmp_pos('x', other),
            self.cmp_pos('y', other),
            self.cmp_pos('z', other),
        )
    }

    fn update_velocity(&mut self, velocity: (i32, i32, i32)) {
        self.velocity = velocity
    }

    fn update_position(&mut self) {
        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
            self.position.2 + self.velocity.2,
        )
    }

    fn potential_energy(self) -> i32 {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }

    fn kinetic_energy(self) -> i32 {
        self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()
    }

    fn total_energy(self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        assert_eq!(179, total_energy(&input, 10));

        let input = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";
        assert_eq!(706, total_energy(&input, 10));
    }

    #[test]
    fn part_2() {
        let input = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

        assert_eq!(BigInt::parse_bytes(b"2772", 10).unwrap(), simulate(&input));

        let input = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";
        assert_eq!(
            BigInt::parse_bytes(b"4686774924", 10).unwrap(),
            simulate(&input)
        );
    }
}
