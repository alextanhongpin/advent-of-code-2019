use intcode::Intcode;
use std::collections::HashMap;
use std::error::Error;

type Packet = (i128, i128);

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("./input.txt");
    let input = input.trim();
    assert_eq!(23954, part1(input));
    assert_eq!(17265, part2(input));

    Ok(())
}

fn part1(input: &str) -> i128 {
    let mut computers = (0..50)
        .map(|address| Intcode::new(input, vec![address, -1]))
        .collect::<Vec<Intcode>>();

    let mut i = 0;

    loop {
        let computer = computers.get_mut(i).unwrap();
        // If not input, set it to -1.
        if computer.input.is_empty() {
            computer.input.push(-1);
        }
        computer.exec();
        let output = computer.output.clone();
        computer.output.clear();
        for i in 0..output.len() / 3 {
            let address = output[i * 3];
            let x = output[i * 3 + 1];
            let y = output[i * 3 + 2];

            if address == 255 {
                return y;
            }

            let computer = computers.get_mut(address as usize).unwrap();
            computer.input.push(x);
            computer.input.push(y);
        }

        i += 1;
        i %= computers.len();
    }
}

fn part2(input: &str) -> i128 {
    let mut computers = (0..50)
        .map(|address| Intcode::new(input, vec![address, -1]))
        .collect::<Vec<Intcode>>();

    let mut i = 0;
    let mut nat: Packet = (0, 0);
    let mut cache: HashMap<Packet, usize> = HashMap::new();
    cache.insert(nat, 0);
    let mut cycle: HashMap<usize, usize> = HashMap::new();

    loop {
        if let Some(num_delivered) = cache.get(&nat) {
            if *num_delivered >= 2 {
                return nat.1;
            }
        }

        let n = computers.len();
        let computer = computers.get_mut(i % n).unwrap();
        if computer.input.is_empty() {
            computer.input.push(-1);

            // Store the idle state.
            let count = cycle.entry(i / n).or_insert(0);
            *count += 1;
        }

        computer.exec();
        let output = computer.output.clone();
        computer.output.clear();
        for i in 0..output.len() / 3 {
            let address = output[i * 3];
            let x = output[i * 3 + 1];
            let y = output[i * 3 + 2];

            if address == 255 {
                nat = (x, y);
            } else {
                let computer = computers.get_mut(address as usize).unwrap();
                computer.input.push(x);
                computer.input.push(y);
            }
        }

        match cycle.get(&(i / n)) {
            Some(&count) if count == n => {
                let packet = cache.entry(nat).or_insert(0);
                *packet += 1;

                let computer = computers.get_mut(0).unwrap();
                computer.input.push(nat.0);
                computer.input.push(nat.1);
            }
            _ => {}
        }

        i += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
