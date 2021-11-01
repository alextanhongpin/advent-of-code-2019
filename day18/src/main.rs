use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();

    let steps = solve(parse(input));
    assert_eq!(81, steps);

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

fn find_by_value(world: &HashMap<Position, Tile>, tile: char) -> Position {
    for (pos, t) in world {
        let t: char = t.clone().into();
        if t == tile {
            return pos.clone();
        }
    }
    panic!("not found");
}

fn find_all_keys(world: &HashMap<Position, Tile>) -> Vec<char> {
    let mut keys: Vec<char> = vec![];
    for (_, tile) in world.iter() {
        match tile {
            Tile::Key(c) => keys.push(c.to_owned()),
            _ => (),
        }
    }
    keys
}

fn find_distance(
    world: &HashMap<Position, Tile>,
    start: char,
    keys: Vec<char>,
    cache: &mut HashMap<(char, char), usize>,
) {
    let mut n_keys = keys.len();
    use Direction::*;
    use Tile::*;

    if keys.iter().all(|&k| cache.contains_key(&(k, start))) {
        return;
    }

    let initial_pos = find_by_value(&world, start);
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(initial_pos);
    let mut moves: Vec<Position> = vec![initial_pos];
    let mut step = 0;

    while moves.len() > 0 || n_keys > 0 {
        let to_move = moves.clone();
        moves.clear();
        step += 1;

        for mv in to_move {
            for dir in vec![Up, Down, Left, Right] {
                let new_pos = mv.move_forward(&dir);
                if visited.contains(&new_pos) {
                    continue;
                }

                match world.get(&new_pos) {
                    Some(Wall) => continue,
                    Some(&Key(c)) => {
                        cache.insert((start, c), step);
                        cache.insert((c, start), step);
                        if n_keys > 0 {
                            n_keys -= 1;
                        }
                    }
                    _ => {}
                }
                visited.insert(new_pos);
                moves.push(new_pos);
            }
        }
    }
}

fn min_path(
    world: &HashMap<Position, Tile>,
    curr_tile: char,
    next_tile: char,
    paths: Vec<Vec<char>>,
    cache: &mut HashMap<(char, char), usize>,
    path_cache: &mut HashMap<String, usize>,
) -> usize {
    let mut cache = cache;
    let mut path_cache = path_cache;

    let steps = match cache.get(&(curr_tile, next_tile)) {
        Some(&steps) => steps,
        None => {
            unimplemented!("steps not found: from {} to {}", curr_tile, next_tile,);
        }
    };

    let paths = paths
        .clone()
        .iter()
        .map(|path| {
            path.into_iter()
                .filter(|&ch| ch != &next_tile && ch != &next_tile.to_ascii_uppercase())
                .map(ToOwned::to_owned)
                .collect::<Vec<char>>()
        })
        .filter(|path| path.len() > 0)
        .collect::<Vec<Vec<char>>>();

    if paths.iter().all(|path| path.len() == 0) {
        return steps;
    }

    let mut path = paths
        .clone()
        .into_iter()
        .flat_map(|p| {
            p.into_iter()
                .filter(|c| c.is_ascii_lowercase())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<char>>();
    path.sort();
    path.dedup();
    let path = path.iter().collect::<String>();
    let key = curr_tile.to_string() + &":" + &next_tile.to_string() + &":" + &path;
    match path_cache.get(&key) {
        Some(&prev_steps) => prev_steps,
        None => {
            let keys = paths
                .clone()
                .into_iter()
                .flat_map(|path| match path.first() {
                    c @ Some('a'..='z') => Some(c.cloned()),
                    _ => None,
                })
                .flatten()
                .collect::<Vec<char>>();

            find_distance(&world, next_tile, keys.clone(), &mut cache);

            let new_steps = keys
                .into_iter()
                .map(|tile| {
                    min_path(
                        &world,
                        next_tile,
                        tile,
                        paths.clone(),
                        &mut cache,
                        &mut path_cache,
                    )
                })
                .min()
                .unwrap()
                + steps;

            path_cache.insert(key, new_steps);
            new_steps
        }
    }
}

fn find_distance2(
    world: &HashMap<Position, Tile>,
    cache: &mut HashMap<String, usize>,
    moved: &mut HashMap<(char, char), usize>,
    start: Position,
    initial_steps: usize,
    start_keys: Vec<char>,
    n_keys: usize,
) {
    if start_keys.len() == n_keys {
        return;
    }
    let tile = world.get(&start).unwrap().clone().into();

    let mut cache = cache;
    let mut moved = moved;
    let mut visited: HashSet<Position> = HashSet::new();
    let mut steps = initial_steps;
    visited.insert(start);
    let mut moves: Vec<(Position, Vec<char>)> = vec![(start, start_keys.clone())];

    use Direction::*;
    use Tile::*;

    while moves.len() > 0 {
        let to_move = moves.clone();
        moves.clear();
        steps += 1;

        for (mv, keys) in to_move {
            for dir in vec![Up, Down, Left, Right] {
                let mut keys = keys.clone();
                let new_pos = mv.move_forward(&dir);
                if visited.contains(&new_pos) {
                    continue;
                }
                visited.insert(new_pos);

                let keep_walking = match world.get(&new_pos) {
                    Some(Wall) => false,
                    Some(&Key(c)) => {
                        if moved.contains_key(&(tile, c)) {
                            continue;
                        }

                        if !keys.contains(&c) {
                            keys.push(c);
                            let key = format!("{}:{}", c, keys.iter().collect::<String>());
                            if !cache.contains_key(&key) {
                                cache.insert(key, steps);

                                moved.insert((tile, c), steps);
                                moved.insert((c, tile), steps);

                                find_distance2(
                                    &world,
                                    &mut cache,
                                    &mut moved,
                                    new_pos,
                                    steps,
                                    keys.clone(),
                                    n_keys,
                                );
                            } else {
                                //println!("cache hit");
                                return;
                            }
                        }
                        keys.len() < n_keys
                    }
                    Some(&Door(c)) => keys.contains(&c.to_ascii_lowercase()),
                    Some(&Empty) | Some(&Player) => true,
                    None => false,
                };
                if !keep_walking {
                    continue;
                }
                moves.push((new_pos, keys));
            }
        }
    }
}

fn solve2(world: HashMap<Position, Tile>) -> usize {
    let mut cache: HashMap<String, usize> = HashMap::new();
    let mut visited: HashSet<Position> = HashSet::new();
    let mut moved: HashMap<(char, char), usize> = HashMap::new();

    let all_keys = find_all_keys(&world);
    let start = find_by_value(&world, '@');
    let mut steps = 0;
    visited.insert(start);
    let mut moves: Vec<(Position, Vec<char>)> = vec![(start, vec![])];

    use Direction::*;
    use Tile::*;
    let tile = '@';

    while moves.len() > 0 {
        let to_move = moves.clone();
        moves.clear();
        steps += 1;

        for (mv, keys) in to_move {
            for dir in vec![Up, Down, Left, Right] {
                let mut keys = keys.clone();
                let new_pos = mv.move_forward(&dir);
                if visited.contains(&new_pos) {
                    continue;
                }
                visited.insert(new_pos);

                let keep_walking = match world.get(&new_pos) {
                    Some(Wall) => false,
                    Some(&Key(c)) => {
                        moved.insert((tile, c), steps);
                        moved.insert((c, tile), steps);
                        if !keys.contains(&c) {
                            keys.push(c);
                            let key = format!("{}:{}", c, keys.iter().collect::<String>());
                            if !cache.contains_key(&key) {
                                cache.insert(key, steps);
                                //find_distance2(
                                //&world,
                                //&mut cache,
                                //&mut moved,
                                //new_pos,
                                //steps,
                                //keys.clone(),
                                //all_keys.len(),
                                //);
                            }
                        }
                        keys.len() < all_keys.len()
                    }
                    _ => true
                    //Some(&Door(c)) => keys.contains(&c.to_ascii_lowercase()),
                    //Some(&Empty) | Some(&Player) => true,
                    //None => false,
                };
                if !keep_walking {
                    continue;
                }
                moves.push((new_pos, keys));
            }
        }
    }
    println!("cache: {:?}", cache);
    println!("{:?}", moved);

    cache
        .into_iter()
        .map(|(key, steps)| {
            let keys = key.split(":").nth(1).unwrap();
            if keys.len() == all_keys.len() {
                Some(steps)
            } else {
                None
            }
        })
        .flatten()
        .min()
        .unwrap()
}

fn solve(world: HashMap<Position, Tile>) -> usize {
    return solve2(world);
    //println!("solver2: {:?}", solve2(world.clone()));
    //let paths = find_paths(world.clone());
    //println!("paths: {:?}", paths);

    //let mut cache: HashMap<(char, char), usize> = HashMap::new();
    //let mut path_cache: HashMap<String, usize> = HashMap::new();
    //let start = '@';

    //let keys = paths
    //.clone()
    //.into_iter()
    //.flat_map(|path| match path.first() {
    //c @ Some('a'..='z') => Some(c.cloned()),
    //_ => None,
    //})
    //.flatten()
    //.collect::<Vec<char>>();

    //find_distance(&world, start, keys.clone(), &mut cache);

    //keys.into_iter()
    //.map(|tile| {
    //min_path(
    //&world,
    //start,
    //tile,
    //paths.clone(),
    //&mut cache,
    //&mut path_cache,
    //)
    //})
    //.min()
    //.unwrap()
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
