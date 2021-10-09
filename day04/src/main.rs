use std::collections::HashMap;

fn main() {
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    for i in 245182..=790572 {
        if valid_password(&i.to_string()) {
            part1 += 1;
        }

        if valid_password_group(&i.to_string()) {
            part2 += 1;
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn parse_password(pwd: &str) -> Option<HashMap<u32, u32>> {
    let mut chars: Vec<char> = pwd.chars().collect();
    chars.sort();

    let sorted: String = chars.into_iter().collect();
    if sorted != pwd {
        return None;
    }

    let mut map: HashMap<u32, u32> = HashMap::new();
    for c in pwd.chars() {
        let count = map.entry(c.to_digit(10).unwrap()).or_insert(0);
        *count += 1;
    }

    Some(map)
}

fn valid_password(pwd: &str) -> bool {
    match parse_password(pwd) {
        Some(chars) => {
            for &v in chars.values() {
                if v >= 2 {
                    return true;
                }
            }
            false
        }
        None => false,
    }
}

fn valid_password_group(pwd: &str) -> bool {
    match parse_password(pwd) {
        Some(chars) => {
            for &v in chars.values() {
                if v == 2 {
                    return true;
                }
            }
            false
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(true, valid_password("111111".into()), "test1");
        assert_eq!(false, valid_password("223450".into()), "test2");
        assert_eq!(false, valid_password("123789".into()), "test3");
    }

    #[test]
    fn part2() {
        assert_eq!(true, valid_password_group("112233".into()), "test1");
        assert_eq!(false, valid_password_group("123444".into()), "test2");
        assert_eq!(true, valid_password_group("111122".into()), "test3");
        assert_eq!(false, valid_password_group("111222".into()), "test4");
        assert_eq!(false, valid_password_group("113393".into()), "test5");
    }
}
