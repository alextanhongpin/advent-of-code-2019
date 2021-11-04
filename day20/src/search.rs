use std::collections::{HashMap, HashSet};

pub type Point = (usize, usize);
pub type Step = usize;
pub type Depth = i32;

type Tile = char;

type Map = HashMap<Point, Tile>;
type Portal = HashMap<Point, Point>;

pub fn find_portals(map: &Map, start: Point, end: Point) -> Vec<(Point, Step, Depth)> {
    let mut portals: Vec<(Point, Step, Depth)> = Vec::new();
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(start);
    let mut queue: Vec<Point> = vec![start];
    let mut steps = 0;

    loop {
        if queue.is_empty() {
            break;
        }

        let moves = queue.clone();
        queue.clear();

        steps += 1;
        for m in moves {
            for curr_move in [
                (m.0, m.1 + 1),
                (m.0, m.1 - 1),
                (m.0 + 1, m.1),
                (m.0 - 1, m.1),
            ] {
                if visited.contains(&curr_move) {
                    continue;
                }

                match map.get(&curr_move) {
                    Some('.') => {
                        queue.push(curr_move);
                        visited.insert(curr_move);
                    }
                    // Outer.
                    Some('%') => {
                        if curr_move == end {
                            portals.push((curr_move, steps, 0));
                        } else {
                            portals.push((curr_move, steps, -1));
                            visited.insert(curr_move);
                        }
                    }
                    // Inner.
                    Some('@') => {
                        portals.push((curr_move, steps, 1));
                        visited.insert(curr_move);
                    }
                    _ => {}
                }
            }
        }
    }

    portals
}

pub fn flood(
    map: &HashMap<(usize, usize), char>,
    start: (usize, usize),
    end: (usize, usize),
    portals: &HashMap<(usize, usize), (usize, usize)>,
) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut moves: Vec<(Point, Step)> = vec![(start, 0)];
    let mut steps: Vec<Step> = vec![];

    loop {
        if moves.is_empty() {
            break;
        }

        let (curr_move, curr_step) = moves.remove(0);
        if visited.contains(&curr_move) {
            continue;
        }

        if curr_move == end {
            steps.push(curr_step);
            continue;
        }

        visited.insert(curr_move.clone());
        let next_moves = find_portals(&map, curr_move, end);
        moves.append(
            &mut next_moves
                .into_iter()
                .map(|(next_move, next_step, _)| match portals.get(&next_move) {
                    Some(&portal_move) => (portal_move, curr_step + next_step + 1),
                    None => (next_move, curr_step + next_step),
                })
                .collect::<Vec<(Point, Step)>>(),
        );
    }

    steps.into_iter().min().unwrap()
}
