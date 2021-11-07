use super::helper::*;
use super::tile::*;
use std::collections::HashMap;
type Map = Vec<Vec<Tile>>;

pub fn part2(input: &str, iter: usize) -> i32 {
    let mut cache: HashMap<Depth, Map> = HashMap::new();
    cache.insert(0, parse(input));
    for _ in 0..iter {
        let mut next_state: Vec<(Depth, Map)> = vec![(0, simulate_one_recursive_round(&cache, 0))];

        // Prevents breaking in the center if the map has no bugs.
        let upper = *cache.keys().max().unwrap();
        let mut max = 0;
        loop {
            max += 1;
            cache.entry(max).or_insert_with(new_map);
            let map = simulate_one_recursive_round(&cache, max);
            if !cache_key(&map).contains('#') && max > upper {
                break;
            }

            next_state.push((max, map));
        }

        // Prevents breaking in the center if the map has no bugs.
        let lower = *cache.keys().min().unwrap();
        let mut min = 0;
        loop {
            min -= 1;
            cache.entry(min).or_insert_with(new_map);
            let map = simulate_one_recursive_round(&cache, min);
            if !cache_key(&map).contains('#') && min < lower {
                break;
            }

            next_state.push((min, map));
        }

        for (depth, map) in next_state {
            cache.insert(depth, map);
        }
    }

    let num_bugs = cache
        .clone()
        .into_iter()
        .flat_map(|(_, map)| map.into_iter().flatten())
        .filter(|&tile| tile == Tile::Bug)
        .count();

    let mut maps = cache
        .into_iter()
        .map(|(depth, map)| (depth, map))
        .collect::<Vec<_>>();
    maps.sort_by(|a, b| a.0.cmp(&b.0));

    for (depth, map) in maps {
        println!("depth {}: \n{}\n", depth, draw(&map, "\n"));
    }

    num_bugs as i32
}

pub fn simulate_one_recursive_round(cache: &HashMap<Depth, Map>, depth: i32) -> Map {
    let mut map = cache.get(&depth).cloned().unwrap();
    let curr = map.clone();

    for (y, row) in curr.into_iter().enumerate() {
        for (x, tile) in row.into_iter().enumerate() {
            // The center tile is recursive.
            if (x, y) == (2, 2) {
                continue;
            }

            match tile {
                Tile::Bug => {
                    let tiles = get_adjacent_recursive_tiles(cache, depth, (x as i32, y as i32));
                    let bug_lives = match tiles.get(&Tile::Bug) {
                        Some(&count) => count == 1,
                        None => false,
                    };
                    if !bug_lives {
                        map[y][x] = Tile::Empty;
                    }
                }
                Tile::Empty => {
                    let tiles = get_adjacent_recursive_tiles(cache, depth, (x as i32, y as i32));
                    let becomes_infested = match tiles.get(&Tile::Bug) {
                        Some(&count) => count == 1 || count == 2,
                        None => false,
                    };
                    if becomes_infested {
                        map[y][x] = Tile::Bug;
                    }
                }
            }
        }
    }

    map
}

fn new_map() -> Map {
    vec![vec![Tile::Empty; 5]; 5]
}

pub fn get_adjacent_recursive_tiles(
    map: &HashMap<Depth, Map>,
    depth: Depth,
    pos: Position,
) -> HashMap<Tile, usize> {
    let upper_map = map.get(&(depth - 1)).cloned().unwrap_or_else(new_map);
    let inner_map = map.get(&(depth + 1)).cloned().unwrap_or_else(new_map);
    let curr_map = map.get(&depth).unwrap();

    let ori_pos = pos;
    let positions = &[
        (pos.0, pos.1 - 1), // Top
        (pos.0 + 1, pos.1), // Right
        (pos.0, pos.1 + 1), // Bottom
        (pos.0 - 1, pos.1), // Left
    ];

    positions
        .iter()
        .flat_map(|&pos| {
            let mut all_pos = vec![];
            // Check one level above.
            if pos.0 < 0 {
                all_pos.push(upper_map[2][1]); // Center left of outer map.
            }

            if pos.0 > 4 {
                all_pos.push(upper_map[2][3]); // Center right of outer map.
            }

            if pos.1 < 0 {
                all_pos.push(upper_map[1][2]); // Center top of outer map.
            }

            if pos.1 > 4 {
                all_pos.push(upper_map[3][2]); // Center bottom of outer map.
            }

            // Check one level below.
            match pos {
                (_, -1) | (-1, _) | (5, _) | (_, 5) => {}
                (2, 2) if ori_pos == (2, 1) => {
                    // Top row of inner map.
                    all_pos.append(&mut inner_map[0].clone());
                }
                (2, 2) if ori_pos == (2, 3) => {
                    // Bottom row of inner map.
                    all_pos.append(&mut inner_map[4].clone());
                }
                (2, 2) if ori_pos == (1, 2) => {
                    // Left column of inner map.
                    all_pos.append(&mut (0..5).map(|y| inner_map[y][0]).collect::<Vec<_>>());
                }
                (2, 2) if ori_pos == (3, 2) => {
                    // Right column of inner map.
                    all_pos.append(&mut (0..5).map(|y| inner_map[y][4]).collect::<Vec<_>>());
                }
                (x, y) => {
                    all_pos.push(curr_map[y as usize][x as usize]);
                }
            }

            all_pos
        })
        .fold(HashMap::new(), |mut cnt, tile| {
            let count = cnt.entry(tile).or_insert(0);
            *count += 1;
            cnt
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "....#
#..#.
#..##
..#..
#....";
        assert_eq!(99, part2(input, 10));
    }
}
