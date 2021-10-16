#[derive(Debug, PartialEq)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl From<i128> for ParameterMode {
    fn from(n: i128) -> Self {
        match n {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => unimplemented!(),
        }
    }
}
