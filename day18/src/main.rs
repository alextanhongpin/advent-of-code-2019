use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

mod direction;
mod position;
mod search;
mod tile;

use direction::*;
use position::*;
use search::*;
use tile::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;
    let input = input.trim();

    assert_eq!(4700, solve(parse(input)));
    assert_eq!(2260, solve2(parse(input)));

    Ok(())
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

fn find_paths(
    world: &HashMap<Position, Tile>,
    start: Position,
    cache: &mut HashMap<(char, char), usize>,
) -> Vec<Vec<char>> {
    let mut visited: HashSet<Position> = HashSet::new();

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

fn solve2(world: HashMap<Position, Tile>) -> usize {
    let mut world = world;

    let tile = '@';
    let mut start = find_by_value(&world, tile);
    world.insert(start, Tile::Wall);
    let mut positions: Vec<Position> = vec![];

    use Direction::*;
    for (i, dir) in vec![Up, Right, Down, Down, Left, Left, Up, Up]
        .iter()
        .enumerate()
    {
        start = start.move_forward(&dir);
        if i % 2 == 1 {
            positions.push(start);
            world.insert(start, Tile::Player);
        } else {
            world.insert(start, Tile::Wall);
        }
    }

    let mut taken: HashMap<String, usize> = HashMap::new();
    let mut cache: HashMap<(char, char), usize> = HashMap::new();
    let all_paths = positions
        .clone()
        .into_iter()
        .map(|pos| {
            (
                '@',
                find_paths(&world, pos, &mut cache)
                    .into_iter()
                    .filter(|path| path.len() > 0)
                    .collect(),
            )
        })
        .collect::<Vec<(char, Vec<Vec<char>>)>>();

    solve_path(&world, &mut taken, &mut cache, all_paths).unwrap()
}

fn path_key(all_paths: &Vec<(char, Vec<Vec<char>>)>) -> String {
    let mut path_key = all_paths
        .clone()
        .into_iter()
        .map(|(start, paths)| subpath_key(start, &paths))
        .filter(|key| key.len() > 0)
        .collect::<Vec<String>>();
    path_key.sort();

    path_key.join(":").trim().to_string()
}

fn subpath_key(start: char, paths: &Vec<Vec<char>>) -> String {
    let mut path_key = paths
        .clone()
        .into_iter()
        .filter(|path| path.len() > 0)
        .map(|path| path.into_iter().collect::<String>())
        .collect::<Vec<String>>();
    path_key.sort();
    if path_key.len() == 0 {
        return "".to_string();
    }
    format!("{}-{}", start, path_key.join(":").trim())
}

fn solve_path(
    world: &HashMap<Position, Tile>,
    taken: &mut HashMap<String, usize>,
    cache: &mut HashMap<(char, char), usize>,
    all_paths: Vec<(char, Vec<Vec<char>>)>,
) -> Option<usize> {
    let mut taken = taken;
    let mut cache = cache;

    if all_paths.clone().into_iter().all(|(_, paths)| {
        paths
            .into_iter()
            .flat_map(|path| path)
            .collect::<Vec<char>>()
            .len()
            == 0
    }) {
        return Some(0);
    }

    let key = path_key(&all_paths);
    match taken.get(&key) {
        Some(&steps) => Some(steps),
        None => {
            match all_paths
                .clone()
                .into_iter()
                .enumerate()
                .flat_map(|(idx, (start, paths))| {
                    let cache_key = format!("{}:{}", start, path_key(&all_paths));
                    match taken.get(&cache_key).cloned() {
                        Some(steps) => Some(steps),
                        None => {
                            let steps = find_nearby_keys(&world, &mut cache, start, paths.clone())
                                .into_iter()
                                .flat_map(|(start, steps)| {
                                    let mut all_paths = all_paths
                                        .clone()
                                        .into_iter()
                                        .map(|(curr, paths)| {
                                            (
                                                curr,
                                                paths
                                                    .into_iter()
                                                    .map(|path| {
                                                        path.into_iter()
                                                            .filter(|&ch| {
                                                                ch != start
                                                                    && ch
                                                                        != start
                                                                            .to_ascii_uppercase()
                                                            })
                                                            .collect::<Vec<char>>()
                                                    })
                                                    .filter(|path| path.len() > 0)
                                                    .collect::<Vec<Vec<char>>>(),
                                            )
                                        })
                                        .collect::<Vec<(char, Vec<Vec<char>>)>>();
                                    all_paths[idx] = (start, all_paths[idx].clone().1);
                                    solve_path(&world, &mut taken, &mut cache, all_paths.clone())
                                        .and_then(|min_steps| Some(min_steps + steps))
                                })
                                .min();
                            match steps {
                                Some(min_steps) => {
                                    taken.insert(cache_key, min_steps);
                                    Some(min_steps)
                                }
                                None => None,
                            }
                        }
                    }
                })
                .min()
            {
                Some(steps) => {
                    taken.insert(key, steps);
                    Some(steps)
                }
                None => None,
            }
        }
    }
}

fn solve(world: HashMap<Position, Tile>) -> usize {
    let mut taken: HashMap<String, usize> = HashMap::new();
    let mut cache: HashMap<(char, char), usize> = HashMap::new();

    let tile = '@';
    let start = find_by_value(&world, tile);
    let paths = find_paths(&world, start, &mut cache);

    find_remaining_keys(&world, &mut taken, &mut cache, tile, paths).unwrap()
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

    #[test]
    fn part2() {
        let input = "#######
        #a.#Cd#
        ##...##
        ##.@.##
        ##...##
        #cB#Ab#
        #######";
        assert_eq!(8, solve2(parse(input)));

        let input = "###############
#d.ABC.#.....a#
######...######
######.@.######
######...######
#b.....#.....c#
###############";
        assert_eq!(24, solve2(parse(input)));

        let input = "#############
        #DcBa.#.GhKl#
        #.###...#I###
        #e#d#.@.#j#k#
        ###C#...###J#
        #fEbA.#.FgHi#
        #############";
        assert_eq!(32, solve2(parse(input)));

        let input = "#############
        #g#f.D#..h#l#
        #F###e#E###.#
        #dCba...BcIJ#
        #####.@.#####
        #nK.L...G...#
        #M###N#H###.#
        #o#m..#i#jk.#
        #############";
        assert_eq!(72, solve2(parse(input)));
    }
}
