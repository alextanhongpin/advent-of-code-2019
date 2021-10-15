#[derive(Debug)]
pub struct Intcode {
    pub input: Vec<i128>,
    pub done: bool,
    pub waiting: bool,
    pub i: usize,
    pub codes: Vec<i128>,
    pub relative_base: i128,
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

fn get_mode(mode: i128, pos: usize) -> ParameterMode {
    let mut mode = mode / 100;
    for _ in 1..pos {
        mode /= 10;
    }
    match mode % 10 {
        0 => ParameterMode::Position,
        1 => ParameterMode::Immediate,
        2 => ParameterMode::Relative,
        n => panic!("invalid parameter mode: {}", n),
    }
}

impl Intcode {
    pub fn new(code: &str, input: Vec<i128>) -> Self {
        let mut input = input;
        input.reverse();

        let codes: Vec<i128> = code
            .split(',')
            .map(|line| line.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        Intcode {
            input: input,
            done: false,
            waiting: false,
            i: 0,
            codes: codes,
            relative_base: 0,
        }
    }

    pub fn set_input(&mut self, input: i128) {
        self.input.insert(0, input);
    }

    fn get_parameter(&mut self, pos: usize, rw: char) -> i128 {
        let n = self.codes[self.i + pos];
        match rw {
            'w' => n,
            'r' => match get_mode(self.codes[self.i], pos) {
                ParameterMode::Position => self.get_memory_at(n as usize),
                ParameterMode::Immediate => n,
                ParameterMode::Relative => n + self.relative_base,
            },
            c => panic!("invalid mode: {}", c),
        }
    }

    fn get_memory_at(&mut self, i: usize) -> i128 {
        if i >= self.codes.len() {
            self.codes.resize(i + 1, 0);
        }
        self.codes[i]
    }

    fn set_memory_at(&mut self, i: usize, n: i128) {
        if i >= self.codes.len() {
            self.codes.resize(i + 1, 0);
        }
        self.codes[i] = n
    }

    fn opcode(&mut self) -> i128 {
        self.codes[self.i] % 100
    }

    pub fn exec(&mut self) -> Option<i128> {
        loop {
            match self.opcode() {
                1 => {
                    let a = self.get_parameter(1, 'r');
                    let b = self.get_parameter(2, 'r');
                    let c = self.get_parameter(3, 'w');
                    self.set_memory_at(c as usize, a + b);
                    self.i += 4;
                }
                2 => {
                    let a = self.get_parameter(1, 'r');
                    let b = self.get_parameter(2, 'r');
                    let c = self.get_parameter(3, 'w');
                    self.set_memory_at(c as usize, a * b);
                    self.i += 4;
                }
                3 => {
                    if self.input.len() == 0 {
                        self.waiting = true;
                        break;
                    }
                    let a = self.get_parameter(1, 'w');
                    self.waiting = false;

                    let n = self.input.pop().unwrap();
                    self.set_memory_at(a as usize, n);
                    self.i += 2;
                }
                4 => {
                    let a = self.get_parameter(1, 'r');
                    self.i += 2;
                    return Some(a);
                }
                5 => {
                    let a = self.get_parameter(1, 'r');
                    let b = self.get_parameter(2, 'w');
                    self.i = if a != 0 { b as usize } else { self.i + 3 }
                }
                6 => {
                    let a = self.get_parameter(1, 'r');
                    let b = self.get_parameter(2, 'w');
                    self.i = if a == 0 { b as usize } else { self.i + 3 }
                }
                7 => {
                    let a = self.get_parameter(1, 'r');
                    let b = self.get_parameter(2, 'r');
                    let c = self.get_parameter(3, 'w');
                    self.set_memory_at(c as usize, if a < b { 1 } else { 0 });
                    self.i += 4;
                }
                8 => {
                    let a = self.get_parameter(1, 'r');
                    let b = self.get_parameter(2, 'r');
                    let c = self.get_parameter(3, 'w');
                    self.set_memory_at(c as usize, if a == b { 1 } else { 0 });
                    self.i += 4;
                }
                9 => {
                    let a = self.get_parameter(1, 'r');
                    self.relative_base += a;
                    self.i += 2;
                }
                99 => {
                    self.done = true;
                    break;
                }
                n => panic!("invalid opcode: {}", n),
            }
        }
        None
    }
}

pub fn program(code: &str, input: Vec<i128>) -> i128 {
    let mut input = input;
    input.reverse();

    let mut codes: Vec<i128> = code
        .split(',')
        .map(|line| line.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let mut i = 0;

    loop {
        let mode = |i: i128, a: i128, b: i128| match i {
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
        assert_eq!(Some(1), Intcode::new("3,0,4,0,99", vec![1]).exec());

        assert_eq!(ParameterMode::Immediate, get_mode(102, 1));
        assert_eq!(ParameterMode::Position, get_mode(1002, 1));
        assert_eq!(ParameterMode::Immediate, get_mode(1002, 2));
        assert_eq!(ParameterMode::Position, get_mode(1002, 3));
    }

    #[test]
    fn part2() {
        let input = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(Some(1), Intcode::new(&input, vec![8]).exec(), "eq8");
        assert_eq!(Some(0), Intcode::new(&input, vec![0]).exec(), "neq8");

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
