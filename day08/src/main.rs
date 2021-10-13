use std::error::Error;
use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const PIXELS: usize = WIDTH * HEIGHT;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    println!("part 1: {}", validate(&input.trim()));

    for row in encode(&input.trim()) {
        println!("{:?}", row);
    }

    Ok(())
}

fn validate(input: &str) -> i32 {
    let num_layers = input.len() / PIXELS;
    // The most number of zeros is equal to the number of pixels.
    let mut min_zeros: i32 = PIXELS as i32;
    let mut one_times_two: i32 = 0;

    // Grab the layers.
    for i in 0..num_layers {
        let start = i * PIXELS;
        let end = (i + 1) * PIXELS;
        let layer = &input[start..end];
        let zeros = layer.matches('0').count() as i32;
        if zeros < min_zeros {
            min_zeros = zeros;
            one_times_two = (layer.matches('1').count() * layer.matches('2').count()) as i32;
        }
    }
    one_times_two
}

fn encode(input: &str) -> Vec<Vec<i32>> {
    let num_layers = input.len() / PIXELS;
    let mut output: Vec<Vec<i32>> = vec![vec![2; WIDTH]; HEIGHT];

    for i in 0..PIXELS {
        match output[i / WIDTH][i % WIDTH] {
            2 => {
                for j in 0..num_layers {
                    match &input[j * PIXELS + i..j * PIXELS + 1 + i] {
                        "2" => continue,
                        n => {
                            output[i / WIDTH][i % WIDTH] = n.parse::<i32>().unwrap();
                            break;
                        }
                    }
                }
            }
            _ => continue,
        }
    }
    output
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
