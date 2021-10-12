use std::error::Error;
use std::fs;

const WIDTH: i32 = 25;
const HEIGHT: i32 = 6;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    println!("part 1: {}", validate(&input.trim()));

    Ok(())
}

fn validate(input: &str) -> i32 {
    let pixels = WIDTH * HEIGHT;

    // The most number of zeros is equal to the number of pixels.
    let mut min_zeros: i32 = pixels;
    let mut one_times_two: i32 = 0;

    // Grab the layers.
    for i in 0..(input.len() as i32 / pixels) {
        let start = (i * pixels) as usize;
        let end = ((i + 1) * pixels) as usize;
        let layer = &input[start..end];
        let zeros = layer.matches('0').count() as i32;
        if zeros < min_zeros {
            min_zeros = zeros;
            one_times_two = (layer.matches('1').count() * layer.matches('2').count()) as i32;
        }
    }
    one_times_two
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
