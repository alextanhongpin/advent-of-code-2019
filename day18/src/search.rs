use std::collections::{HashMap, HashSet};

use super::position::Position;
use super::tile::Tile;

pub fn find_nearby_keys(
    world: &HashMap<Position, Tile>,
    cache: &mut HashMap<(char, char), usize>,
    start: char,
    paths: Vec<Vec<char>>,
) -> Vec<(char, usize)> {
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
        return vec![];
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
            let steps = cache.get(&(start, key)).unwrap_or(&0).clone();
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
            Some((key, steps))
        })
        .flatten()
        .collect()
}

pub fn find_remaining_keys(
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
            let steps = cache.get(&(start, key)).unwrap_or(&0).clone();
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
                return Some(taken.get(&cache_key).cloned().unwrap());
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

    use super::direction::Direction::*;
    use super::tile::Tile::*;

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

pub fn find_by_value(world: &HashMap<Position, Tile>, tile: char) -> Position {
    for (pos, t) in world {
        let t: char = t.clone().into();
        if t == tile {
            return pos.clone();
        }
    }
    panic!("not found");
}

fn draw(world: HashMap<Position, Tile>) {
    let max_x = world.keys().map(|p| p.x).max().unwrap() as usize;
    let max_y = world.keys().map(|p| p.y).max().unwrap() as usize;

    let mut coordinates: Vec<Vec<String>> = vec![vec![".".to_string(); max_x + 1]; max_y + 1];
    for (pos, tile) in world.iter() {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let tile: char = tile.clone().into();
        coordinates[y][x] = tile.to_string();
    }

    for row in coordinates {
        println!("{:?}", row.join(""));
    }
    println!("");
}
