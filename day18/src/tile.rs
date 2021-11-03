#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Player,
    Empty,
    Wall,
    Key(char),
    Door(char),
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Player => '@',
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Key(c) => c,
            Tile::Door(c) => c,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            '@' => Tile::Player,
            'a'..='z' => Tile::Key(c),
            'A'..='Z' => Tile::Door(c),
            _ => panic!("Unknown tile: {}", c),
        }
    }
}
