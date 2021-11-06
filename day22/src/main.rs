fn main() {
    let input = include_str!("./input.txt");
    println!("{}", play2(input, 10_007, 2019));
    assert_eq!(2558, part1(input));
}

fn part1(input: &str) -> usize {
    play(input, 10_007)
        .into_iter()
        .position(|n| n == 2019)
        .unwrap()
}

enum Step {
    DealIntoNewStack,
    Cut(i32),
    DealWithIncrement(i32),
}

fn play2(input: &str, n_cards: usize, i: usize) -> usize {
    use Step::*;
    let mut i = i;

    for line in input.lines() {
        let step = parse_step(line);
        i = match step {
            DealIntoNewStack => deal_into_new_stack_index(i, n_cards),
            DealWithIncrement(increment) => {
                deal_with_increment_n_index(i as i32, increment, n_cards as i32)
            }
            Cut(cut) => cut_n_cards_index(i, cut, n_cards),
        };
    }

    i
}

fn play(input: &str, n_cards: usize) -> Vec<usize> {
    use Step::*;
    let mut cards = (0..n_cards).collect::<Vec<usize>>();

    for line in input.lines() {
        let step = parse_step(line);
        cards = match step {
            DealIntoNewStack => deal_into_new_stack(&cards),
            DealWithIncrement(increment) => deal_with_increment_n(&cards, increment),
            Cut(cut) => cut_n_cards(&cards, cut),
        };
    }

    cards
}

fn parse_step(input: &str) -> Step {
    use Step::*;

    let input = input.trim();
    if input.contains("new") {
        DealIntoNewStack
    } else if input.contains("increment") {
        let increment = input
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        DealWithIncrement(increment)
    } else if input.contains("cut") {
        let cut = input
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        Cut(cut)
    } else {
        panic!("Unknown step: {}", input)
    }
}

fn deal_into_new_stack_index(i: usize, n_cards: usize) -> usize {
    n_cards - 1 - i
}

fn deal_into_new_stack(cards: &[usize]) -> Vec<usize> {
    (0..cards.len())
        .map(|i| cards[deal_into_new_stack_index(i, cards.len())])
        .collect()
}

fn cut_n_cards_index(i: usize, n: i32, n_cards: usize) -> usize {
    let n = ((n as i32 + n_cards as i32) % n_cards as i32) as usize;
    (i + n as usize) % n_cards
}

fn cut_n_cards(cards: &[usize], n: i32) -> Vec<usize> {
    (0..cards.len())
        .map(|i| cards[cut_n_cards_index(i, n, cards.len())])
        .collect()
}

fn deal_with_increment_n_index(i: i32, n: i32, n_cards: i32) -> usize {
    (i as i128 * multiplicative_inverse(n as i128, n_cards as i128) % n_cards as i128) as usize
}

fn deal_with_increment_n(cards: &[usize], n: i32) -> Vec<usize> {
    (0..cards.len())
        .map(|i| cards[deal_with_increment_n_index(i as i32, n, cards.len() as i32)])
        .collect()
}

// https://github.com/Aidiakapi/advent_of_code_2019/blob/master/src/day22.rs
// Calculates the multiplicative inverse in a finite field.
// Based on psuedocode in: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures
fn multiplicative_inverse(a: i128, n: i128) -> i128 {
    let mut t = 0i128;
    let mut newt = 1i128;
    let mut r = n;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;
        t -= quotient * newt;
        r -= quotient * newr;
        std::mem::swap(&mut t, &mut newt);
        std::mem::swap(&mut r, &mut newr);
    }

    if r > 1 {
        panic!("invalid n");
    }
    if t < 0 {
        t += n;
    }

    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal_into_new_stack() {
        assert_eq!(99, deal_into_new_stack_index(0, 100));
        assert_eq!(0, deal_into_new_stack_index(99, 100));
        assert_eq!(19, deal_into_new_stack_index(80, 100));

        let cards = (0..9).collect::<Vec<usize>>();
        let cards = deal_into_new_stack(&cards);
        assert_eq!(vec![8, 7, 6, 5, 4, 3, 2, 1, 0], cards);
    }

    #[test]
    fn test_cut_n_cards() {
        let cards = (0..10).collect::<Vec<usize>>();
        assert_eq!(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2], cut_n_cards(&cards, 3));
        assert_eq!(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5], cut_n_cards(&cards, -4));
    }

    #[test]
    fn test_deal_with_increment_n() {
        let cards = (0..10).collect::<Vec<usize>>();
        assert_eq!(
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
            deal_with_increment_n(&cards, 3)
        );
    }

    #[test]
    fn test_play() {
        let cards = (0..10).collect::<Vec<usize>>();
        let cards = deal_with_increment_n(&cards, 7);
        let cards = deal_into_new_stack(&cards);
        let cards = deal_into_new_stack(&cards);
        assert_eq!(vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7], cards);

        let cards = (0..10).collect::<Vec<usize>>();
        let cards = cut_n_cards(&cards, 6);
        let cards = deal_with_increment_n(&cards, 7);
        let cards = deal_into_new_stack(&cards);
        assert_eq!(vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6], cards,);

        let cards = (0..10).collect::<Vec<usize>>();
        let cards = deal_with_increment_n(&cards, 7);
        let cards = deal_with_increment_n(&cards, 9);
        let cards = cut_n_cards(&cards, -2);
        assert_eq!(vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9], cards,);

        let cards = (0..10).collect::<Vec<usize>>();
        let cards = deal_into_new_stack(&cards);
        let cards = cut_n_cards(&cards, -2);
        let cards = deal_with_increment_n(&cards, 7);
        let cards = cut_n_cards(&cards, 8);
        let cards = cut_n_cards(&cards, -4);
        let cards = deal_with_increment_n(&cards, 7);
        let cards = cut_n_cards(&cards, 3);
        let cards = deal_with_increment_n(&cards, 9);
        let cards = deal_with_increment_n(&cards, 3);
        let cards = cut_n_cards(&cards, -1);
        assert_eq!(vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6], cards,);
    }

    #[test]
    fn test_play_instructions() {
        let instruction = "deal with increment 7
deal into new stack
deal into new stack";
        assert_eq!(vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7], play(instruction, 10));

        let instruction = "cut 6
deal with increment 7
deal into new stack";
        assert_eq!(vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6], play(instruction, 10));

        let instruction = "deal with increment 7
deal with increment 9
cut -2";
        assert_eq!(vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9], play(instruction, 10));

        let instruction = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
        assert_eq!(vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6], play(instruction, 10));
    }
}
