fn main() {
    let input = include_str!("./input.txt");
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

fn play(input: &str, number_of_cards: usize) -> Vec<usize> {
    use Step::*;
    let mut cards = (0..number_of_cards).collect::<Vec<usize>>();

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

fn deal_into_new_stack(cards: &[usize]) -> Vec<usize> {
    let mut cards = cards.to_vec();
    cards.reverse();
    cards
}

fn cut_n_cards(cards: &[usize], n: i32) -> Vec<usize> {
    if n < 0 {
        cut_n_cards(cards, (cards.len() as i32) + n)
    } else {
        let head = &cards[0..(n as usize)].to_owned();
        let mut head = head.to_vec();

        let tail = &cards[(n as usize)..].to_owned();
        let mut tail = tail.to_vec();

        tail.append(&mut head);
        tail.to_owned()
    }
}

fn deal_with_increment_n(cards: &[usize], n: i32) -> Vec<usize> {
    let mut stack = vec![0; cards.len()];
    stack[0] = cards[0];
    let mut i = 1;
    let mut j = 0;

    while i < cards.len() {
        j += n;
        j %= cards.len() as i32;
        stack[j as usize] = cards[i];
        i += 1;
    }

    stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal_into_new_stack() {
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
