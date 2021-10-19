use regex::Regex;
use std::cmp::Ordering;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    assert_eq!(11384, total_energy(&input, 1000));
    Ok(())
}

fn total_energy(input: &str, steps: i32) -> i32 {
    let mut moons = Moon::from(input.to_string());
    for _ in 0..steps {
        for j in 0..4 {
            moons.apply_gravity(j);
        }
        for j in 0..4 {
            moons.apply_velocity(j);
        }
    }

    let total_energy = (0..4).map(|n| moons.total_energy(n)).fold(0, |a, b| a + b);
    total_energy
}
#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn energy(self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug)]
struct Moon {
    positions: Vec<Position>,
    velocities: Vec<Position>,
}

fn parse_row(input: &str) -> Position {
    let re = Regex::new(r"<x=(-?\d+),\s*y=(-?\d+),\s*z=(-?\d+)\s*>").unwrap();
    let cap = re.captures(input).unwrap();
    Position {
        x: cap[1].parse::<i32>().unwrap(),
        y: cap[2].parse::<i32>().unwrap(),
        z: cap[3].parse::<i32>().unwrap(),
    }
}

fn parse(input: &str) -> Vec<Position> {
    let input = input.trim();
    input
        .split('\n')
        .map(|row| parse_row(row))
        .collect::<Vec<Position>>()
}

impl From<String> for Moon {
    fn from(input: String) -> Moon {
        Moon::new(parse(&input))
    }
}

impl Moon {
    fn new(positions: Vec<Position>) -> Self {
        let len = positions.len();
        Moon {
            positions: positions,
            velocities: vec![Position { x: 0, y: 0, z: 0 }; len],
        }
    }

    fn apply_gravity(&mut self, moon: usize) {
        let curr_pos = self.positions[moon];
        let mut vel = self.velocities[moon];
        for (i, pos) in self.positions.iter().enumerate() {
            if i == moon {
                continue;
            }
            match curr_pos.x.cmp(&pos.x) {
                Ordering::Less => vel.x += 1,
                Ordering::Greater => vel.x -= 1,
                Ordering::Equal => {}
            }

            match curr_pos.y.cmp(&pos.y) {
                Ordering::Less => vel.y += 1,
                Ordering::Greater => vel.y -= 1,
                Ordering::Equal => {}
            }

            match curr_pos.z.cmp(&pos.z) {
                Ordering::Less => vel.z += 1,
                Ordering::Greater => vel.z -= 1,
                Ordering::Equal => {}
            }
        }
        self.velocities[moon] = vel;
    }

    fn apply_velocity(&mut self, moon: usize) {
        let mut pos = self.positions[moon];
        let vel = self.velocities[moon];
        pos.x += vel.x;
        pos.y += vel.y;
        pos.z += vel.z;
        self.positions[moon] = pos;
    }

    fn potential_energy(&self, moon: usize) -> i32 {
        self.positions[moon].energy()
    }

    fn kinetic_energy(&self, moon: usize) -> i32 {
        self.velocities[moon].energy()
    }

    fn total_energy(&self, moon: usize) -> i32 {
        self.potential_energy(moon) * self.kinetic_energy(moon)
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
}
