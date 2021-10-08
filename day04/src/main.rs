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

fn valid_password(pwd: &str) -> bool {
    let mut has_adjacent = false;
    let mut chars = pwd.chars();
    let mut prev: u32 = chars.next().unwrap().to_digit(10).unwrap();

    for n in chars {
        let n = n.to_digit(10).unwrap();
        match n {
            _ if n < prev => return false,
            _ if n == prev => {
                if !has_adjacent {
                    has_adjacent = true;
                }
            }
            _ => {}
        }
        prev = n;
    }

    has_adjacent
}

fn valid_password_group(pwd: &str) -> bool {
    let mut counter: HashMap<u32, u32> = HashMap::new();

    let mut chars = pwd.chars();
    let mut prev: u32 = chars.next().unwrap().to_digit(10).unwrap();

    for n in chars {
        let n = n.to_digit(10).unwrap();
        match n {
            _ if n < prev => return false,
            m if n == prev => {
                let count = counter.entry(m).or_insert(1);
                *count += 1;
            }
            _ => {}
        }
        prev = n;
    }

    for (_, &v) in counter.iter() {
        if v == 2 {
            return true;
        }
    }

    false
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
