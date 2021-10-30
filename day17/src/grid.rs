use super::Direction;

use regex::Regex;
use std::collections::HashMap;

pub fn draw_map(input: Vec<i128>) {
    let pixels = ascii(input);

    let (mut max_x, mut max_y) = (0, 0);
    for (&(x, y), _) in pixels.iter() {
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    let mut out: Vec<Vec<String>> = vec![vec![".".to_string(); max_x + 1]; max_y + 1];
    for (&(x, y), px) in pixels.iter() {
        out[y][x] = px.to_owned();
    }

    for row in out {
        println!("{:?}", row.join(""));
    }
}

pub fn ascii(pixels: Vec<i128>) -> HashMap<(usize, usize), String> {
    let mut result: HashMap<(usize, usize), String> = HashMap::new();
    let mut pos: (usize, usize) = (0, 0);
    for px in pixels {
        match px {
            10 => {
                pos.0 = 0;
                pos.1 += 1;
                continue;
            }
            c => {
                result.insert(pos, (c as u8 as char).to_string());
            }
        }
        pos.0 += 1;
    }

    result
}

pub fn to_ascii(input: &str) -> Vec<i128> {
    input.chars().map(|c| c as i128).collect::<Vec<i128>>()
}

// Splits R10L5 to [R,10,L,5,\n]
pub fn to_routine(input: &str) -> Vec<i128> {
    let re = Regex::new(r"([R|L])(\d+)").unwrap();
    let mut out = re
        .captures_iter(input)
        .flat_map(|cap| {
            vec![
                cap[1].to_owned(),
                ",".to_string(),
                cap[2].to_owned(),
                ",".to_string(),
            ]
        })
        .flat_map(|s| s.chars().collect::<Vec<char>>())
        .map(|c| c as i128)
        .collect::<Vec<i128>>();
    out.pop();
    out.push('\n' as i128);
    out
}

// compress_path compresses the path into a mapping of the function to the path.
pub fn compress_path(steps: Vec<Direction>) -> Vec<(HashMap<String, String>, String)> {
    let mut result: Vec<(HashMap<String, String>, String)> = vec![];

    let path = steps
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("");

    let n = steps.len();
    for i in 1..=10 {
        if i > n {
            continue;
        }

        // Find C.
        let c = &steps[n - i..]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("");

        let mut abs = path.split(c).filter(|s| s.len() > 0).collect::<Vec<&str>>();
        abs.sort();

        for j in 1..=10 {
            if i + j > n {
                continue;
            }
            // Find A and B.
            let a = &steps[n - i - j..n - i]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join("");

            let mut dedup: HashMap<String, String> = HashMap::new();
            dedup.insert(c.to_string(), "C".to_string());

            let a_or_b = |is_a| {
                if is_a {
                    "A".to_string()
                } else {
                    "B".to_string()
                }
            };

            // The first pattern must be A.
            let is_a = path.starts_with(a);
            dedup.insert(a.to_string(), a_or_b(is_a));

            for ab in abs.iter() {
                // Remove all instances of A in the string.
                // The remaining string must be B.
                let mut sub = ab.split(a).filter(|s| s.len() > 0).collect::<Vec<&str>>();
                sub.dedup();
                if sub.len() != 1 {
                    continue;
                };
                let b = sub[0];
                dedup.insert(b.to_string(), a_or_b(!is_a));
            }

            // A, B and C must be present.
            if dedup.len() != 3 {
                continue;
            }

            let mut path = path.clone();
            for (k, v) in dedup.iter() {
                path = path.replace(&k.clone(), &v.clone());
            }

            // Include the mapping A => R10L10 from R10L10 => A.
            for (k, v) in dedup.clone() {
                dedup.insert(v, k);
            }

            result.push((dedup, path));
        }
    }

    result
}
