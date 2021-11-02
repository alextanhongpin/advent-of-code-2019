use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();

    let steps = solve(parse(input));
    assert_eq!(4700, steps);

    Ok(())
}

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

fn find_by_value(world: &HashMap<Position, Tile>, tile: char) -> Position {
    for (pos, t) in world {
        let t: char = t.clone().into();
        if t == tile {
            return pos.clone();
        }
    }
    panic!("not found");
}

fn find_distance(
    world: &HashMap<Position, Tile>,
    cache: &mut HashMap<(char, char), usize>,
    start: Position,
) {
    let tile = world.get(&start).unwrap().clone().into();

    let mut visited: HashSet<Position> = HashSet::new();
    let mut steps = 0;
    visited.insert(start);
    let mut moves: Vec<Position> = vec![start];

    use Direction::*;
    use Tile::*;

    while moves.len() > 0 {
        let to_move = moves.clone();
        moves.clear();
        steps += 1;

        for mv in to_move {
            for dir in vec![Up, Down, Left, Right] {
                let new_pos = mv.move_forward(&dir);
                if visited.contains(&new_pos) {
                    continue;
                }
                visited.insert(new_pos);

                let keep_walking = match world.get(&new_pos) {
                    Some(Wall) => false,
                    Some(&Key(c)) => {
                        cache.insert((tile, c), steps);
                        cache.insert((c, tile), steps);
                        true
                    }
                    Some(&Door(_)) | Some(&Empty) | Some(&Player) => true,
                    None => false,
                };
                if !keep_walking {
                    continue;
                }
                moves.push(new_pos);
            }
        }
    }
}

fn find_paths(
    world: &HashMap<Position, Tile>,
    cache: &mut HashMap<(char, char), usize>,
) -> Vec<Vec<char>> {
    let mut visited: HashSet<Position> = HashSet::new();

    let start = find_by_value(&world, '@');
    let mut steps = 0;
    visited.insert(start);
    let mut moves: Vec<(Position, Vec<char>)> = vec![(start, vec![])];

    use Direction::*;
    use Tile::*;
    let tile = '@';
    let mut paths: Vec<Vec<char>> = vec![];

    while moves.len() > 0 {
        let to_move = moves.clone();
        moves.clear();
        steps += 1;

        for (mv, keys) in to_move {
            let mut walls = 0;
            for dir in vec![Up, Down, Left, Right] {
                let mut keys = keys.clone();
                let new_pos = mv.move_forward(&dir);
                if visited.contains(&new_pos) {
                    if let Some(Wall) = world.get(&new_pos) {
                        walls += 1;
                    }
                    continue;
                }
                visited.insert(new_pos);

                let keep_walking = match world.get(&new_pos) {
                    Some(Wall) => {
                        walls += 1;
                        false
                    }
                    Some(&Key(c)) => {
                        cache.insert((tile, c), steps);
                        cache.insert((c, tile), steps);
                        keys.push(c);
                        true
                    }
                    Some(&Door(c)) => {
                        keys.push(c);
                        true
                    }
                    Some(&Empty) | Some(&Player) => true,
                    None => false,
                };
                if !keep_walking {
                    continue;
                }
                moves.push((new_pos, keys));
            }
            if walls == 3 {
                paths.push(keys);
            }
        }
    }
    paths
}

fn find_remaining_keys(
    world: &HashMap<Position, Tile>,
    taken: &mut HashMap<String, usize>,
    cache: &mut HashMap<(char, char), usize>,
    start: char,
    paths: Vec<Vec<char>>,
) -> Option<usize> {
    let mut taken = taken;
    let mut cache = cache;

    let keys_to_visit = paths
        .iter()
        .flat_map(|path| match path.first() {
            c @ Some('a'..='z') => Some(c),
            _ => None,
        })
        .flatten()
        .collect::<Vec<&char>>();

    if keys_to_visit.len() == 0 {
        return Some(0);
    }

    if !keys_to_visit
        .iter()
        .all(|&key| cache.contains_key(&(start, *key)))
    {
        find_distance(&world, &mut cache, find_by_value(&world, start));
    }

    keys_to_visit
        .into_iter()
        .map(|&key| {
            let steps = cache.get(&(start, key)).unwrap().clone();
            let paths = paths
                .clone()
                .into_iter()
                .map(|path| {
                    path.into_iter()
                        .filter(|&ch| ch != key && ch != key.to_ascii_uppercase())
                        .collect::<Vec<char>>()
                })
                .filter(|path| path.len() > 0)
                .collect::<Vec<Vec<char>>>();

            let mut path_key = paths
                .clone()
                .into_iter()
                .map(|path| path.into_iter().collect::<String>())
                .collect::<Vec<String>>();
            path_key.sort();
            let cache_key = format!("{}:{}:{}", start, key, path_key.join(":"));
            if taken.contains_key(&cache_key) {
                return taken.get(&cache_key).cloned();
            }

            let min_steps = find_remaining_keys(&world, &mut taken, &mut cache, key, paths.clone())
                .and_then(|s| Some(steps + s))
                .unwrap();

            taken.insert(cache_key, min_steps);

            Some(min_steps)
        })
        .flatten()
        .min()
}

fn solve(world: HashMap<Position, Tile>) -> usize {
    let mut taken: HashMap<String, usize> = HashMap::new();
    let mut cache: HashMap<(char, char), usize> = HashMap::new();

    let paths = find_paths(&world, &mut cache);
    let tile = '@';

    let keys_to_visit = paths
        .iter()
        .flat_map(|path| match path.first() {
            c @ Some('a'..='z') => Some(c),
            _ => None,
        })
        .flatten()
        .collect::<Vec<&char>>();

    let min = keys_to_visit
        .into_iter()
        .map(|&key| {
            let steps = cache.get(&(tile, key)).unwrap().clone();
            let paths = paths
                .clone()
                .into_iter()
                .map(|path| {
                    path.into_iter()
                        .filter(|&ch| ch != key && ch != key.to_ascii_uppercase())
                        .collect::<Vec<char>>()
                })
                .filter(|path| path.len() > 0)
                .collect::<Vec<Vec<char>>>();

            let mut path_key = paths
                .clone()
                .into_iter()
                .map(|path| path.into_iter().collect::<String>())
                .collect::<Vec<String>>();
            path_key.sort();
            let cache_key = format!("{}:{}:{}", tile, key, path_key.join(":"));
            if taken.contains_key(&cache_key) {
                return taken.get(&cache_key).cloned();
            }

            let min_steps = find_remaining_keys(&world, &mut taken, &mut cache, key, paths)
                .and_then(|s| Some(steps + s))
                .unwrap();

            taken.insert(cache_key, min_steps);

            Some(min_steps)
        })
        .flatten()
        .min()
        .unwrap();

    min
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
