pub fn program(code: &str, input: Vec<i32>) -> i32 {
    let mut input = input;
    input.reverse();

    let mut codes: Vec<i32> = code
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut i = 0;

    loop {
        let mode = |i: i32, a: i32, b: i32| match i {
            _ if i > 1100 => (a, b),
            _ if i > 1000 => (codes[a as usize], b),
            _ if i > 100 => (a, codes[b as usize]),
            _ => (codes[a as usize], codes[b as usize]),
        };
        match codes[i] % 100 {
            1 => {
                if let [c, a, b, p] = codes[i..i + 4] {
                    let (a, b) = mode(c, a, b);
                    codes[p as usize] = a + b;
                    i += 4;
                }
            }
            2 => {
                if let [c, a, b, p] = codes[i..i + 4] {
                    let (a, b) = mode(c, a, b);
                    codes[p as usize] = a * b;
                    i += 4;
                }
            }
            3 => {
                if let [_, p] = codes[i..i + 2] {
                    codes[p as usize] = input.pop().unwrap();
                    i += 2;
                }
            }
            4 => {
                if let [c, p] = codes[i..i + 2] {
                    return match c {
                        4 => codes[p as usize],
                        _ => p,
                    };
                }
            }
            5 => {
                if let [c, a, b] = codes[i..i + 3] {
                    let (a, b) = mode(c, a, b);
                    i = if a != 0 { b as usize } else { i + 3 }
                }
            }
            6 => {
                if let [c, a, b] = codes[i..i + 3] {
                    let (a, b) = mode(c, a, b);
                    i = if a == 0 { b as usize } else { i + 3 }
                }
            }
            7 => {
                if let [c, a, b, p] = codes[i..i + 4] {
                    let (a, b) = mode(c, a, b);
                    codes[p as usize] = if a < b { 1 } else { 0 };
                    i += 4;
                }
            }
            8 => {
                if let [c, a, b, p] = codes[i..i + 4] {
                    let (a, b) = mode(c, a, b);
                    codes[p as usize] = if a == b { 1 } else { 0 };
                    i += 4;
                }
            }
            99 => break,
            _ => panic!("invalid"),
        }
    }

    panic!("invalid program")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(1, program("3,0,4,0,99", vec![1]));
    }

    #[test]
    fn part2() {
        let input = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(1, program(&input, vec![8]), "eq8");
        assert_eq!(0, program(&input, vec![0]), "neq8");

        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(1, program(&input, vec![0]), "lt8");
        assert_eq!(0, program(&input, vec![8]), "eq8");

        let input = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(1, program(&input, vec![8]), "eq8");
        assert_eq!(0, program(&input, vec![0]), "neq8");

        let input = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(1, program(&input, vec![0]), "lt8");
        assert_eq!(0, program(&input, vec![8]), "eq8");

        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(0, program(&input, vec![0]), "zero");
        assert_eq!(1, program(&input, vec![1]), "non-zero");

        let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(0, program(&input, vec![0]), "zero");
        assert_eq!(1, program(&input, vec![1]), "non-zero");

        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(999, program(&input, vec![7]), "lt8");
        assert_eq!(1000, program(&input, vec![8]), "eq8");
        assert_eq!(1001, program(&input, vec![9]), "gt8");
    }
}
