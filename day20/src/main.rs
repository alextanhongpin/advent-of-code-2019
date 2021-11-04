use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    assert_eq!(664, solve(&input));
    assert_eq!(7334, part2(&input));

    Ok(())
}

fn parse(
    input: &str,
) -> (
    HashMap<(usize, usize), char>,
    (usize, usize),
    (usize, usize),
    HashMap<(usize, usize), (usize, usize)>,
) {
    let mut map = HashMap::new();
    let positions = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let x = positions[0].len();
    let y = positions.len();

    let mut portals: HashMap<(char, char), Vec<(usize, usize)>> = HashMap::new();

    for j in 0..y {
        // Move from left to right.
        for i in 0..x {
            let c = positions[j][i];
            match c {
                portal @ 'A'..='Z' => {
                    if i + 1 >= x {
                        continue;
                    }
                    // Is a vertical portal.
                    if !positions[j][i + 1].is_ascii_uppercase() {
                        continue;
                    }

                    let portal_name = || {
                        if i == 0 || i == x - 2 {
                            // Outer.
                            '%'
                        } else {
                            // Inner.
                            '@'
                        }
                    };

                    let portal = (portal, positions[j][i + 1]);
                    // Find the direction of the portal.
                    if i + 2 < x && positions[j][i + 2] == '.' {
                        portals
                            .entry(portal)
                            .or_insert_with(Vec::new)
                            .push((i + 2, j));
                        map.insert((i + 2, j), portal_name());
                    } else {
                        portals
                            .entry(portal)
                            .or_insert_with(Vec::new)
                            .push((i - 1, j));
                        map.insert((i - 1, j), portal_name());
                    }
                }
                '#' | '.' => {
                    map.entry((i, j)).or_insert(c);
                }
                _ => {}
            }
        }
    }
    for i in 0..x {
        for j in 0..y {
            // Move from left to right.
            if let portal @ 'A'..='Z' = positions[j][i] {
                if j + 1 >= y {
                    continue;
                }
                // Is a horizontal portal.
                if !positions[j + 1][i].is_ascii_uppercase() {
                    continue;
                }

                let portal_name = || {
                    if j == 0 || j == y - 2 {
                        // Outer.
                        '%'
                    } else {
                        // Inner.
                        '@'
                    }
                };
                let portal = (portal, positions[j + 1][i]);
                // Find the direction of the portal.
                if j + 2 < y && positions[j + 2][i] == '.' {
                    portals
                        .entry(portal)
                        .or_insert_with(Vec::new)
                        .push((i, j + 2));
                    map.insert((i, j + 2), portal_name());
                } else {
                    portals
                        .entry(portal)
                        .or_insert_with(Vec::new)
                        .push((i, j - 1));
                    map.insert((i, j - 1), portal_name());
                }
            }
        }
    }

    // Flatten the portals. If is bidirectional, then A -> B and B -> A.
    // Otherwise, collect them as start and end portal, AA and ZZ.
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut teleport: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    for (doors, positions) in portals {
        match doors {
            ('A', 'A') => {
                start = positions[0];
            }
            ('Z', 'Z') => {
                end = positions[0];
            }
            _ => {
                teleport.insert(positions[0], positions[1]);
                teleport.insert(positions[1], positions[0]);
            }
        }
    }

    (map, start, end, teleport)
}

fn draw(map: &HashMap<(usize, usize), char>) {
    let x = map.keys().map(|(x, _)| *x).max().unwrap();
    let y = map.keys().map(|(_, y)| *y).max().unwrap();

    let mut grid = vec![vec![' '; x + 1]; y + 1];
    for (pos, c) in map {
        grid[pos.1][pos.0] = *c;
    }

    for row in grid {
        println!("{}", row.into_iter().collect::<String>());
    }
}

fn solve(input: &str) -> usize {
    let (map, start, end, portals) = parse(input);
    draw(&map);
    flood(&map, start, end, &portals)
}

fn part2(input: &str) -> usize {
    let (map, start, end, portals) = parse(input);
    draw(&map);
    depth(&map, start, end, &portals)
}

fn flood(
    map: &HashMap<(usize, usize), char>,
    start: (usize, usize),
    end: (usize, usize),
    portals: &HashMap<(usize, usize), (usize, usize)>,
) -> usize {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut moves: Vec<((usize, usize), usize)> = vec![(start, 1)];
    let mut all_steps: Vec<usize> = vec![];

    loop {
        if moves.is_empty() {
            break;
        }

        let next_moves = moves.clone();
        moves.clear();

        for (m, steps) in next_moves {
            for dir in [
                (m.0, m.1 + 1),
                (m.0, m.1 - 1),
                (m.0 + 1, m.1),
                (m.0 - 1, m.1),
            ] {
                if visited.contains(&dir) {
                    continue;
                }

                match map.get(&dir) {
                    Some('.') => {
                        moves.push((dir, steps + 1));
                        visited.insert(dir);
                    }
                    Some('#') => {}
                    Some('@') | Some('%') => {
                        if dir == end {
                            all_steps.push(steps);
                            break;
                        }
                        if portals.contains_key(&dir) {
                            let next = portals.get(&dir).unwrap();
                            moves.push((*next, steps + 2));
                            visited.insert(*next);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    all_steps.into_iter().min().unwrap()
}

fn depth(
    map: &HashMap<(usize, usize), char>,
    start: (usize, usize),
    end: (usize, usize),
    portals: &HashMap<(usize, usize), (usize, usize)>,
) -> usize {
    let mut visited: HashSet<((usize, usize), usize)> = HashSet::new();
    let mut moves: Vec<(
        (usize, usize),
        usize,
        usize,
        HashSet<((usize, usize), usize)>,
    )> = vec![(start, 0, 1, visited.clone())];
    let mut all_steps: Vec<usize> = vec![];

    loop {
        if moves.is_empty() {
            break;
        }

        let next_moves = moves.clone();
        moves.clear();

        for (m, depth, steps, doors) in next_moves {
            for dir in [
                (m.0, m.1 + 1),
                (m.0, m.1 - 1),
                (m.0 + 1, m.1),
                (m.0 - 1, m.1),
            ] {
                if visited.contains(&(dir, depth))
                    || (depth > 1
                        && (1..depth)
                            .into_iter()
                            .filter(|&d| doors.contains(&(dir, d)))
                            .count()
                            > 5)
                {
                    continue;
                }

                match map.get(&dir) {
                    Some('.') => {
                        moves.push((dir, depth, steps + 1, doors.clone()));
                        visited.insert((dir, depth));
                    }
                    Some('#') => {}
                    Some(&inout) if inout == '@' || inout == '%' => {
                        // When the depth is 1, the outer layer START and END functions as wall.
                        if depth != 0 && inout == '%' && (dir == start || dir == end) {
                            continue;
                        }
                        // When the depth is 0, only the outer layer START and END functions.
                        if depth == 0 && inout == '%' && (dir != start && dir != end) {
                            continue;
                        }
                        if dir == end {
                            println!("found end: {}", steps);
                            all_steps.push(steps);
                            break;
                        }
                        let up_or_down = || {
                            // Inner layer increases depth by 1.
                            if inout == '@' {
                                depth + 1
                            } else {
                                // Outer layer decreases depth by 1.
                                depth - 1
                            }
                        };
                        if portals.contains_key(&dir) {
                            let mut doors = doors.clone();
                            doors.insert((dir, depth));

                            let next = portals.get(&dir).unwrap();
                            let depth = up_or_down();

                            visited.insert((*next, depth));
                            doors.insert((*next, depth));
                            moves.push((*next, depth, steps + 2, doors.clone()));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    all_steps.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
";
        assert_eq!(23, solve(input));

        let input = "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";
        assert_eq!(58, solve(input));
    }

    #[test]
    fn test2() {
        let input = "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
";
        assert_eq!(26, part2(input));

        let input = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";

        assert_eq!(396, part2(input));
    }
}
