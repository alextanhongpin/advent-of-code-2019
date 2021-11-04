use std::collections::HashMap;
use std::error::Error;
use std::fs;

mod search;
use search::*;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("./src/input.txt")?;

    assert_eq!(664, part1(&input));
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

fn part1(input: &str) -> usize {
    let (map, start, end, portals) = parse(input);
    draw(&map);

    flood(&map, start, end, &portals)
}

fn part2(input: &str) -> usize {
    let (map, start, end, portals) = parse(input);
    draw(&map);
    flood2(&map, start, end, &portals)
}

fn flood2(
    map: &HashMap<(usize, usize), char>,
    start: (usize, usize),
    end: (usize, usize),
    portals: &HashMap<(usize, usize), (usize, usize)>,
) -> usize {
    let mut cache: HashMap<Point, Vec<(Point, Step, Depth)>> = HashMap::new();
    let mut visited: HashMap<Point, Depth> = HashMap::new();
    let mut moves: Vec<(Point, Step, Depth)> = vec![(start, 0, 0)];
    let mut steps: Vec<Step> = vec![];

    loop {
        if moves.is_empty() {
            break;
        }

        let (curr_move, curr_step, curr_depth) = moves.remove(0);
        if curr_depth < 0 {
            continue;
        }

        // This large number is used to check how much did the path repeats itself.
        // I don't have any idea on what heuristic to check against infinite recursion, so I just
        // use a large number.
        if *visited.entry(curr_move).or_insert(curr_depth) > 3000 {
            continue;
        }

        let is_start_or_end = curr_move == start || curr_move == end;
        match map.get(&curr_move) {
            Some('%') => {
                if curr_depth == 0 && !is_start_or_end {
                    continue;
                }
                if curr_depth > 0 && is_start_or_end {
                    continue;
                }
            }
            _ => {}
        }

        if curr_depth == 0 && curr_move == end {
            steps.push(curr_step);
            continue;
        }

        *visited.entry(curr_move).or_insert(curr_depth) += 1;

        let next_moves = if cache.contains_key(&curr_move) {
            cache.get(&curr_move).unwrap().clone()
        } else {
            let next_moves = find_portals(&map, curr_move, end);
            cache.insert(curr_move, next_moves.clone());
            next_moves
        };
        moves.append(
            &mut next_moves
                .into_iter()
                .map(
                    |(next_move, next_step, depth)| match portals.get(&next_move) {
                        Some(&portal_move) => {
                            (portal_move, curr_step + next_step + 1, curr_depth + depth)
                        }
                        None => (next_move, curr_step + next_step, curr_depth + depth),
                    },
                )
                .collect::<Vec<(Point, Step, Depth)>>(),
        );
    }
    steps.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
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
        assert_eq!(23, part1(input));

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
        assert_eq!(58, part1(input));
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
