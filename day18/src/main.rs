use std::collections::HashMap;

fn main() {}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Right => Left,
            Left => Right,
        }
    }
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x: x, y: y }
    }

    fn move_forward(self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Position::new(self.x, self.y - 1),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Player,
    Empty,
    Wall,
    Key(char),
    Door(char),
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

fn parse(input: &str) -> HashMap<Position, Tile> {
    let input = input.trim();

    let mut map = HashMap::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.trim().chars() {
            map.insert(Position::new(x, y), Tile::from(c));
            x += 1;
        }
        y += 1;
    }
    map
}

fn draw(world: HashMap<Position, Tile>) {
    let max_x = world.keys().map(|p| p.x).max().unwrap() as usize;
    let max_y = world.keys().map(|p| p.y).max().unwrap() as usize;

    use Tile::*;
    let mut coordinates: Vec<Vec<String>> = vec![vec![".".to_string(); max_x + 1]; max_y + 1];
    for (pos, tile) in world.iter() {
        let x = pos.x as usize;
        let y = pos.y as usize;
        coordinates[y][x] = match tile {
            Player => "P".to_string(),
            Empty => ".".to_string(),
            Wall => "#".to_string(),
            Key(c) => c.to_string(),
            Door(c) => c.to_string(),
        };
    }

    for row in coordinates {
        println!("{:?}", row.join(""));
    }
    println!("");
}

fn walk(
    world: HashMap<Position, Tile>,
    keys: usize,
    direction: Direction,
    position: Position,
) -> usize {
    use Direction::*;
    use Tile::*;
    let mut world = world;
    if vec![Up, Down, Left, Right]
        .into_iter()
        .filter(|dir| dir != &direction)
        .all(|dir| {
            world.get(
                &position
                    .move_forward(&direction.opposite())
                    .move_forward(&dir),
            ) == Some(&Wall)
        })
    {
        world.insert(position.move_forward(&direction.opposite()), Wall);
    }

    let position = position.move_forward(&direction);

    match world.get(&position) {
        Some(Empty) => vec![Up, Down, Left, Right]
            .into_iter()
            .filter(|&dir| dir != direction.opposite())
            .map(|direction| walk(world.clone(), keys, direction, position) + 1)
            .min()
            .unwrap(),
        Some(Key(c)) => {
            if keys == 1 {
                // We have the last key.
                return 1;
            }
            let mut world = world.clone();
            // Remove the key.
            world.remove(&position);
            world.insert(
                position,
                match world.get(&position.move_forward(&direction)) {
                    Some(Wall) => Wall,
                    _ => Empty,
                },
            );

            println!(
                "keys: {}, key: {:?}, door: {:?}",
                keys,
                c,
                c.to_uppercase()
                    .collect::<Vec<char>>()
                    .first()
                    .unwrap()
                    .to_owned()
            );

            // Remove the door if exists.
            match world.iter().find(|&(_, v)| {
                v == &Door(
                    c.to_uppercase()
                        .collect::<Vec<char>>()
                        .first()
                        .unwrap()
                        .to_owned(),
                )
            }) {
                Some((&pos, _)) => {
                    world.remove(&pos);
                    world.insert(pos, Empty);
                }
                _ => {}
            }

            draw(world.clone());

            vec![Up, Down, Left, Right]
                .into_iter()
                .map(|direction| walk(world.clone(), keys - 1, direction, position))
                .min()
                .unwrap()
                + 1
        }
        Some(Wall) | Some(Door(_)) => usize::max_value() / 2,
        s => panic!("unknown step: {:?} {:?}", s, position),
    }
}

fn solve(world: HashMap<Position, Tile>) -> usize {
    let mut world = world;
    // Find the start position.
    let start = world
        .clone()
        .into_iter()
        .find(|(_, v)| v == &Tile::Player)
        .unwrap()
        .0;
    world.remove(&start);
    world.insert(start, Tile::Empty);

    println!("start: {:?}", start);
    let keys = world
        .iter()
        .map(|(_, v)| match v {
            Tile::Key(_) => 1,
            _ => 0,
        })
        .sum();

    use Direction::*;
    vec![Up, Down, Left, Right]
        .into_iter()
        .map(|direction| {
            let world = world.clone();
            let position = start.clone();
            walk(world, keys, direction, position)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "#########
#b.A.@.a#
#########";
        let steps = solve(parse(input));
        assert_eq!(8, steps);

        let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        let steps = solve(parse(input));
        assert_eq!(86, steps);

        let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

        let steps = solve(parse(input));
        assert_eq!(132, steps);

        let input = "#################
        #i.G..c...e..H.p#
        ########.########
        #j.A..b...f..D.o#
        ########@########
        #k.E..a...g..B.n#
        ########.########
        #l.F..d...h..C.m#
        #################";

        let steps = solve(parse(input));
        assert_eq!(136, steps);

        let input = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

        let steps = solve(parse(input));
        assert_eq!(81, steps);
    }
}
