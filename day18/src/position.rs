use super::direction::Direction;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x: x, y: y }
    }

    pub fn move_forward(self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Position::new(self.x, self.y - 1),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }
}
