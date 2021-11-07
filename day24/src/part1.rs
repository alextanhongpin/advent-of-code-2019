use super::helper::*;
use super::tile::*;
use std::collections::HashMap;

pub fn part1(input: &str) -> i32 {
    let mut map = parse(input);
    let mut cache: HashMap<String, usize> = HashMap::new();
    loop {
        let key = cache_key(&map);
        let cnt = cache.entry(key.clone()).or_insert(0);
        *cnt += 1;
        if *cnt == 2 {
            let rating = key
                .match_indices('#')
                .map(|(pos, _)| 2_i32.pow(pos as u32))
                .sum::<i32>();
            return rating;
        }
        simulate_one_round(&mut map);
    }
}

pub fn simulate_one_round(map: &mut Vec<Vec<Tile>>) {
    let curr = map.clone();
    for (y, row) in curr.clone().into_iter().enumerate() {
        for (x, tile) in row.into_iter().enumerate() {
            match tile {
                Tile::Bug => {
                    let tiles = get_adjacent_tiles(&curr, (x as i32, y as i32));
                    let bug_lives = match tiles.get(&Tile::Bug) {
                        Some(&count) => count == 1,
                        None => false,
                    };
                    if !bug_lives {
                        map[y][x] = Tile::Empty;
                    }
                }
                Tile::Empty => {
                    let tiles = get_adjacent_tiles(&curr, (x as i32, y as i32));
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
}

pub fn get_adjacent_tiles(map: &[Vec<Tile>], pos: Position) -> HashMap<Tile, usize> {
    let positions = &[
        (pos.0, pos.1 - 1), // Top
        (pos.0 + 1, pos.1), // Right
        (pos.0, pos.1 + 1), // Bottom
        (pos.0 - 1, pos.1), // Left
    ];

    positions
        .iter()
        .filter(|pos| pos.0 >= 0 && pos.1 >= 0)
        .map(|pos| {
            map.get(pos.1 as usize)
                .and_then(|row| row.get(pos.0 as usize))
        })
        .flatten()
        .fold(HashMap::new(), |mut cnt, &tile| {
            let count = cnt.entry(tile).or_insert(0);
            *count += 1;
            cnt
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "....#
#..#.
#..##
..#..
#....";
        assert_eq!(2129920, part1(input));
    }
}
