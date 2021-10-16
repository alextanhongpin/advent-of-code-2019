mod operation;
mod parameter_mode;
mod signal;

use operation::Operation;
use parameter_mode::ParameterMode;
pub use signal::Signal;

#[derive(Debug)]
pub struct Intcode {
    base: i128,
    input: Vec<i128>,
    ip: usize,
    pub codes: Vec<i128>,
    pub output: Vec<i128>,
}

fn get_mode(mode: i128, pos: usize) -> ParameterMode {
    let mut mode = mode / 100;
    for _ in 1..pos {
        mode /= 10;
    }
    ParameterMode::from(mode % 10)
}

impl Intcode {
    pub fn parse(code: &str) -> Vec<i128> {
        code.split(',')
            .map(|line| line.parse::<i128>().unwrap())
            .collect::<Vec<i128>>()
    }

    pub fn new(code: &str, input: Vec<i128>) -> Self {
        Intcode {
            input: input,
            ip: 0,
            codes: Intcode::parse(code),
            base: 0,
            output: Vec::new(),
        }
    }

    pub fn set_input(&mut self, input: i128) {
        self.input.insert(0, input);
    }

    fn get_parameter(&mut self, pos: usize, rw: char) -> i128 {
        let n = self.read(self.ip + pos);
        let mode = get_mode(self.codes[self.ip], pos);
        match rw {
            'w' => match mode {
                ParameterMode::Relative => n + self.base,
                _ => n,
            },
            'r' => match mode {
                ParameterMode::Position => self.read(n as usize),
                ParameterMode::Immediate => n,
                ParameterMode::Relative => self.read((n + self.base) as usize),
            },
            c => unimplemented!("invalid: {}", c),
        }
    }

    fn read(&mut self, ip: usize) -> i128 {
        if ip >= self.codes.len() {
            0
            //self.codes.resize(ip + 1, 0);
        } else {
            self.codes[ip as usize]
        }
    }

    fn write(&mut self, ip: usize, n: i128) {
        if ip >= self.codes.len() {
            self.codes.resize(ip + 1, 0);
        }
        self.codes[ip] = n
    }

    fn opcode(&mut self) -> Operation {
        let n = self.codes[self.ip] % 100;
        let op = Operation::from(n);
        op
    }

    pub fn exec(&mut self) -> Signal {
        loop {
            match self.opcode() {
                Operation::Add => {
                    let a = self.get_parameter(1, 'r');
                    let b = self.get_parameter(2, 'r');
                    let c = self.get_parameter(3, 'w');
                    self.write(c as usize, a + b);
                    self.ip += 4;
                }
                Operation::Mul => {
                    let a = self.get_parameter(1, 'r');
                    let b = self.get_parameter(2, 'r');
                    let c = self.get_parameter(3, 'w');
                    self.write(c as usize, a * b);
                    self.ip += 4;
                }
                Operation::Input => {
                    if self.input.is_empty() {
                        return Signal::Waiting;
                    }
                    let a = self.get_parameter(1, 'w');
                    let n = self.input.remove(0);
                    self.write(a as usize, n);
                    self.ip += 2;
                }
                Operation::Output => {
                    let a = self.get_parameter(1, 'r');
                    self.output.push(a);
                    self.ip += 2;
                }
                Operation::JumpIfTrue => {
                    let a = self.get_parameter(1, 'r');
                    let b = self.get_parameter(2, 'r');
                    self.ip = if a != 0 { b as usize } else { self.ip + 3 }
                }
                Operation::JumpIfFalse => {
                    let a = self.get_parameter(1, 'r');
                    let b = self.get_parameter(2, 'r');
                    self.ip = if a == 0 { b as usize } else { self.ip + 3 }
                }
                Operation::LessThan => {
                    let a = self.get_parameter(1, 'r');
                    let b = self.get_parameter(2, 'r');
                    let c = self.get_parameter(3, 'w');
                    self.write(c as usize, if a < b { 1 } else { 0 });
                    self.ip += 4;
                }
                Operation::Equal => {
                    let a = self.get_parameter(1, 'r');
                    let b = self.get_parameter(2, 'r');
                    let c = self.get_parameter(3, 'w');
                    self.write(c as usize, if a == b { 1 } else { 0 });
                    self.ip += 4;
                }
                Operation::RelativeBase => {
                    let a = self.get_parameter(1, 'r');
                    self.base += a;
                    self.ip += 2;
                }
                Operation::Halt => {
                    return Signal::Halt;
                }
            }
        }
    }

    pub fn run_until_halt(&mut self) -> Vec<i128> {
        loop {
            match self.exec() {
                Signal::Halt => break,
                Signal::Waiting => continue,
            }
        }
        self.output.clone()
    }
}

impl From<Vec<i128>> for Intcode {
    fn from(codes: Vec<i128>) -> Self {
        Intcode {
            input: Vec::new(),
            ip: 0,
            codes: codes,
            base: 0,
            output: Vec::new(),
        }
    }
}

impl From<String> for Intcode {
    fn from(codes: String) -> Self {
        Intcode {
            input: Vec::new(),
            ip: 0,
            codes: Intcode::parse(codes.trim().into()),
            base: 0,
            output: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part1() {
        let mut program = Intcode::new("3,0,4,0,99", vec![1]);
        program.run_until_halt();
        assert_eq!(vec![1], program.output, "output eq input");

        assert_eq!(ParameterMode::Position, get_mode(2, 1));
        assert_eq!(ParameterMode::Immediate, get_mode(102, 1));
        assert_eq!(ParameterMode::Relative, get_mode(202, 1));
        assert_eq!(ParameterMode::Position, get_mode(2, 2));
        assert_eq!(ParameterMode::Position, get_mode(102, 2));
        assert_eq!(ParameterMode::Position, get_mode(202, 2));

        assert_eq!(ParameterMode::Position, get_mode(1002, 1));
        assert_eq!(ParameterMode::Immediate, get_mode(1002, 2));
        assert_eq!(ParameterMode::Position, get_mode(1002, 3));
    }

    #[test]
    fn part2() {
        let exec = |code: &str, input: i128| {
            let mut program = Intcode::new(code, vec![input]);
            let output = program.run_until_halt();
            *output.first().unwrap()
        };

        // Input 1.
        let input = "3,9,8,9,10,9,4,9,99,-1,8";
        assert_eq!(1, exec(&input, 8), "eq8");
        assert_eq!(0, exec(&input, 0), "neq8");

        // Input 2.
        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        assert_eq!(1, exec(&input, 0), "lt8");
        assert_eq!(0, exec(&input, 8), "eq8");

        // Input 3.
        let input = "3,3,1108,-1,8,3,4,3,99";
        assert_eq!(1, exec(&input, 8), "eq8");
        assert_eq!(0, exec(&input, 0), "neq8");

        let input = "3,3,1107,-1,8,3,4,3,99";
        assert_eq!(1, exec(&input, 0), "lt8");
        assert_eq!(0, exec(&input, 8), "eq8");

        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        assert_eq!(0, exec(&input, 0), "zero");
        assert_eq!(1, exec(&input, 1), "non-zero");

        let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        assert_eq!(0, exec(&input, 0), "zero");
        assert_eq!(1, exec(&input, 1), "non-zero");

        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        assert_eq!(999, exec(&input, 7), "lt8");
        assert_eq!(1000, exec(&input, 8), "eq8");
        assert_eq!(1001, exec(&input, 9), "gt8");
    }
}
