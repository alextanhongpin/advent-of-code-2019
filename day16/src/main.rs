use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();
    let pattern = vec![0, 1, 0, -1];

    let mut result = run_in_phase(num_vector(&input), pattern.clone(), 100);
    result.truncate(8);
    assert_eq!(vec![1, 0, 3, 3, 2, 4, 4, 7], result);

    let mut result = handle_real_signal(num_vector(&input), pattern.clone(), 100);
    result.truncate(8);
    assert_eq!(vec![1, 4, 2, 8, 8, 0, 2, 5], result);

    Ok(())
}

fn vector_num(input: Vec<i32>) -> i32 {
    input
        .iter()
        .enumerate()
        .map(|(i, n)| (10_i32.pow((input.len() - i - 1) as u32) * n) as i32)
        .sum::<i32>()
}

fn num_vector(input: &str) -> Vec<i32> {
    input
        .trim()
        .chars()
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect()
}

fn handle_real_signal(input: Vec<i32>, pattern: Vec<i32>, num_phases: i32) -> Vec<i32> {
    let offset = vector_num(input.clone().into_iter().take(7).collect::<Vec<i32>>());
    let mut output = input.repeat(10_000);
    for _ in 0..num_phases {
        output = flawed_frequency_transmission(output, pattern.clone(), offset as usize);
    }

    output
        .into_iter()
        .cycle()
        .skip(offset as usize)
        .take(8)
        .collect::<Vec<i32>>()
}

fn run_in_phase(input: Vec<i32>, pattern: Vec<i32>, num_phases: i32) -> Vec<i32> {
    let mut output = input;
    for _ in 0..num_phases {
        output = flawed_frequency_transmission(output, pattern.clone(), 0);
    }
    output
}

fn flawed_frequency_transmission(input: Vec<i32>, pattern: Vec<i32>, offset: usize) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; input.len()];
    let mid = input.len() / 2;

    if offset > mid {
        let total = input[offset - 1..].iter().sum::<i32>();
        result[offset - 1] = total;
    }

    for i in offset..input.len() {
        let mut total = 0;
        if i < mid {
            let mut p = 0;
            let mut j = 0;

            loop {
                let patt = pattern[p % pattern.len()];
                let repeat = if p == 0 { i } else { i + 1 };
                let min = j;
                let max = (j + repeat).min(input.len());
                if patt != 0 {
                    total += patt * input[min..max].iter().sum::<i32>();
                }
                if max == input.len() {
                    break;
                }
                j = max;
                p += 1;
            }
            result[i] = total;
        } else if i == mid {
            result[i] = input[mid..].iter().sum::<i32>();
        } else {
            result[i] = result[i - 1] - input[i - 1];
        }
    }

    result.iter().map(|n| n.abs() % 10).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let pattern = vec![0, 1, 0, -1];
        let result = run_in_phase(input.clone(), pattern.clone(), 1);
        assert_eq!(vec![4, 8, 2, 2, 6, 1, 5, 8], result);

        let result = run_in_phase(input.clone(), pattern.clone(), 2);
        assert_eq!(vec![3, 4, 0, 4, 0, 4, 3, 8], result);

        let result = run_in_phase(input.clone(), pattern.clone(), 3);
        assert_eq!(vec![0, 3, 4, 1, 5, 5, 1, 8], result);

        let result = run_in_phase(input.clone(), pattern.clone(), 4);
        assert_eq!(vec![0, 1, 0, 2, 9, 4, 9, 8], result);

        let mut result = run_in_phase(
            num_vector("80871224585914546619083218645595"),
            pattern.clone(),
            100,
        );
        result.truncate(8);
        assert_eq!(vec![2, 4, 1, 7, 6, 1, 7, 6], result);

        let mut result = run_in_phase(
            num_vector("19617804207202209144916044189917"),
            pattern.clone(),
            100,
        );
        result.truncate(8);
        assert_eq!(vec![7, 3, 7, 4, 5, 4, 1, 8], result);

        let mut result = run_in_phase(
            num_vector("69317163492948606335995924319873"),
            pattern.clone(),
            100,
        );
        result.truncate(8);
        assert_eq!(vec![5, 2, 4, 3, 2, 1, 3, 3], result);
    }

    #[test]
    fn part2() {
        let pattern = vec![0, 1, 0, -1];
        let mut result = handle_real_signal(
            num_vector("03036732577212944063491565474664"),
            pattern.clone(),
            100,
        );
        result.truncate(8);
        assert_eq!(vec![8, 4, 4, 6, 2, 0, 2, 6], result);

        let mut result = handle_real_signal(
            num_vector("02935109699940807407585447034323"),
            pattern.clone(),
            100,
        );
        result.truncate(8);
        assert_eq!(vec![7, 8, 7, 2, 5, 2, 7, 0], result);

        let mut result = handle_real_signal(
            num_vector("03081770884921959731165446850517"),
            pattern.clone(),
            100,
        );
        result.truncate(8);
        assert_eq!(vec![5, 3, 5, 5, 3, 7, 3, 1], result);
    }
}
