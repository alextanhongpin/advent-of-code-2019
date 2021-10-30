use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    UP(usize),
    DOWN(usize),
    LEFT(usize),
    RIGHT(usize),
}

impl Direction {
    pub fn steps(self) -> usize {
        use Direction::*;
        match self {
            UP(n) | DOWN(n) | LEFT(n) | RIGHT(n) => n,
        }
    }
}

// Prints UP(10) to U10.
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Direction::*;
        match self {
            UP(n) => write!(f, "U{:?}", n),
            DOWN(n) => write!(f, "D{:?}", n),
            LEFT(n) => write!(f, "L{:?}", n),
            RIGHT(n) => write!(f, "R{:?}", n),
        }
    }
}
