use intcode::{Intcode, Signal};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();

    let drone = Intcode::from(input.to_string());

    let (steps, drone) = find_shortest_path(drone);
    assert_eq!(226, steps);

    let (steps, _) = find_shortest_path(drone);
    assert_eq!(343, steps);

    Ok(())
}

fn backtrack(i: i32) -> i32 {
    match i {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        n => unimplemented!("n: {}", n),
    }
}

fn find_shortest_path(drone: Intcode) -> (i32, Intcode) {
    let mut depth = 0;

    // Do a breadth-first search by deploying the drones to all possible directions.
    let mut drones: Vec<(i32, Intcode)> = vec![(-1, drone.clone())];

    loop {
        // Take drones for the current batch.
        let programs = drones.clone();
        drones.clear();

        // Deploy to all directions.
        for (prev, program) in programs {
            for i in 1..=4 {
                // Don't let the drone move backward, only move forward.
                if backtrack(i) == prev {
                    continue;
                }
                let mut program = program.clone();
                program.set_input(i as i128);
                match program.exec() {
                    Signal::Waiting => {
                        let status = program.output.pop().unwrap();
                        match status {
                            0 => {
                                //println!("hit roadblock, {:?}", drones.len());
                            }
                            1 => {
                                //println!("can move");
                                drones.push((i as i32, program));
                            }
                            2 => return (depth + 1, program),
                            _ => unimplemented!(),
                        }
                    }
                    Signal::Halt => (),
                }
            }
        }

        depth += 1;
        // Part 2 is easier. Start from the oxygen tank, and keep walking until there are no more
        // paths.
        if drones.len() == 0 {
            return (depth, drone);
        }
    }
}
