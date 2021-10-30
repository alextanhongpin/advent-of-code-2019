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
            35 => {
                result.insert(pos, "#".to_string());
            }
            46 => {
                result.insert(pos, ".".to_string());
            }
            10 => {
                pos.0 = 0;
                pos.1 += 1;
                continue;
            }
            94 => {
                result.insert(pos, "^".to_string());
            }
            n => {
                result.insert(pos, (n as u8 as char).to_string());
            }
        }
        pos.0 += 1;
    }

    result
}
