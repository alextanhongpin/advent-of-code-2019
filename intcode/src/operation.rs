#[derive(Debug, PartialEq)]
pub enum Operation {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equal = 8,
    RelativeBase = 9,
    Halt = 99,
}

impl From<i128> for Operation {
    fn from(n: i128) -> Self {
        match n {
            1 => Operation::Add,
            2 => Operation::Mul,
            3 => Operation::Input,
            4 => Operation::Output,
            5 => Operation::JumpIfTrue,
            6 => Operation::JumpIfFalse,
            7 => Operation::LessThan,
            8 => Operation::Equal,
            9 => Operation::RelativeBase,
            99 => Operation::Halt,
            _ => unimplemented!("invalid operation: {}", n),
        }
    }
}
