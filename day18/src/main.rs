use std::collections::{HashMap, HashSet};

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
            Player => "@".to_string(),
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

fn find_paths(world: HashMap<Position, Tile>) -> Vec<Vec<char>> {
    let start = world
        .clone()
        .into_iter()
        .find(|(_, v)| v == &Tile::Player)
        .unwrap()
        .0;

    let mut paths: Vec<(Position, Direction, Vec<char>)> = vec![
        (start.clone(), Up, vec![]),
        (start.clone(), Down, vec![]),
        (start.clone(), Left, vec![]),
        (start.clone(), Right, vec![]),
    ];

    use Direction::*;
    use Tile::*;
    let mut subpaths: Vec<Vec<char>> = vec![];
    let mut visited: HashSet<Position> = HashSet::new();

    while paths.len() > 0 {
        let head = paths.remove(0);
        let pos = head.0;
        let dir = head.1;
        let mut path = head.2;

        if visited.contains(&pos) {
            println!("visited: {:?}", pos);
            continue;
        }

        // Check if it's junction.
        if vec![Up, Down, Left, Right]
            .into_iter()
            .flat_map(|dir| world.get(&pos.move_forward(&dir)))
            .filter(|&tile| tile == &Tile::Wall)
            .count()
            == 3
            && pos != start
        {
            visited.insert(pos);
            subpaths.push(path);
            continue;
        }

        let new_pos = pos.move_forward(&dir);
        let keep_walking = match world.get(&new_pos) {
            Some(&Key(c)) | Some(&Door(c)) => {
                path.push(c);
                true
            }
            Some(Empty) => true,
            _ => false,
        };
        if keep_walking {
            vec![Up, Down, Left, Right]
                .into_iter()
                .filter(|&direction| direction != dir.opposite())
                .for_each(|dir| {
                    paths.push((new_pos, dir, path.clone()));
                })
        }
    }
    subpaths
}

fn find_by_value(world: HashMap<Position, Tile>, tile: char) -> Position {
    for (pos, t) in world.iter() {
        if t == &Tile::from(tile) {
            return pos.clone();
        }
    }
    panic!("not found");
}

fn distance_from(
    world: HashMap<Position, Tile>,
    dir: Direction,
    curr: Position,
    next: Position,
) -> Option<usize> {
    use Direction::*;
    if curr == next {
        return Some(0);
    }

    vec![Up, Down, Left, Right]
        .into_iter()
        .filter(|direction| direction != &dir.opposite())
        .flat_map(|dir| {
            let new_pos = curr.move_forward(&dir);
            if new_pos == next {
                return Some(1);
            };

            match world.get(&new_pos) {
                Some(&Tile::Wall) => None,
                _ => distance_from(world.clone(), dir, new_pos, next)
                    .and_then(|steps| Some(steps + 1)),
            }
        })
        .min()
}

fn min_path(
    world: &HashMap<Position, Tile>,
    curr_tile: char,
    next_tile: char,
    paths: Vec<Vec<char>>,
    cache: &mut HashMap<(char, char), usize>,
) -> usize {
    let mut cache = cache;
    let steps = match cache.get(&(curr_tile, next_tile)) {
        Some(&steps) => steps,
        None => {
            let curr_position = find_by_value(world.clone(), curr_tile);
            let next_position = find_by_value(world.clone(), next_tile);

            use Direction::*;
            let steps = vec![Up, Down, Left, Right]
                .into_iter()
                .flat_map(|dir| distance_from(world.clone(), dir, curr_position, next_position))
                .min()
                .unwrap();
            cache.insert((next_tile, curr_tile), steps);
            cache.insert((curr_tile, next_tile), steps);
            steps
        }
    };

    let paths = paths
        .clone()
        .iter()
        .map(|path| {
            path.into_iter()
                .filter(|&ch| {
                    ch != &(next_tile)
                        && ch
                            != (next_tile
                                .to_string()
                                .to_uppercase()
                                .chars()
                                .collect::<Vec<char>>()
                                .first()
                                .unwrap())
                })
                .map(|ch| ch.to_owned())
                .collect::<Vec<char>>()
        })
        .filter(|path| path.len() > 0)
        .collect::<Vec<Vec<char>>>();

    if paths.iter().all(|path| path.len() == 0) {
        return steps;
    }

    let keys = paths
        .clone()
        .into_iter()
        .flat_map(|path| match path.first() {
            c @ Some('a'..='z') => Some(c.cloned()),
            _ => None,
        })
        .flatten()
        .collect::<Vec<char>>();

    keys.into_iter()
        .map(|tile| min_path(&world, next_tile, tile, paths.clone(), &mut cache))
        .min()
        .unwrap()
        + steps
}

fn solve(world: HashMap<Position, Tile>) -> usize {
    let paths = find_paths(world.clone());
    println!("paths: {:?}", paths);

    let mut cache: HashMap<(char, char), usize> = HashMap::new();
    let start = '@';

    let keys = paths
        .clone()
        .into_iter()
        .flat_map(|path| match path.first() {
            c @ Some('a'..='z') => Some(c.cloned()),
            _ => None,
        })
        .flatten()
        .collect::<Vec<char>>();
    //println!("keys: {:?}", keys);

    keys.into_iter()
        .map(|tile| min_path(&world, start, tile, paths.clone(), &mut cache))
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
