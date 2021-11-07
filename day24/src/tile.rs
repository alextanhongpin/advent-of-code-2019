pub type Position = (i32, i32);
pub type Depth = i32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Bug,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Bug,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

impl From<Tile> for char {
    fn from(t: Tile) -> Self {
        match t {
            Tile::Empty => '.',
            Tile::Bug => '#',
        }
    }
}
