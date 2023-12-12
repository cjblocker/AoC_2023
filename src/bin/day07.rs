//! Day 7: Camel Cards
use std::convert::From;
use std::env;
use std::fs::read_to_string;

type Cards = [u64; 5];

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    order: u64,
    cards: Cards,
    bid: u64,
}

impl From<&str> for Hand {
    fn from(item: &str) -> Self {
        let (cards, bid) = item.split_once(' ').unwrap();
        let cards: Cards = cards
            .chars()
            .map(|c| match c {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Unrecognized Card"),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let order = determine_order_p1(&cards);
        let bid = bid.parse().unwrap();
        Self { order, cards, bid }
    }
}

fn determine_order_p1(cards: &Cards) -> u64 {
    let mut counts = [0u8; 15];
    for card in cards {
        counts[*card as usize] += 1;
    }
    counts.sort();
    match counts
        .into_iter()
        .rev()
        .take(2)
        .collect::<Vec<_>>()
        .as_slice()
    {
        [5, 0] => 6, // 5 of a Kind
        [4, 1] => 5, // 4 of a Kind
        [3, 2] => 4, // Full House
        [3, 1] => 3, // 3 of a Kind
        [2, 2] => 2, // Two Pairs
        [2, 1] => 1, // 2 of a Kind
        [1, 1] => 0, // High Card
        a => panic!("Unknown hand type: {a:?}"),
    }
}

impl Hand {
    fn from_p2(item: &str) -> Self {
        let (cards, bid) = item.split_once(' ').unwrap();
        let cards: Cards = cards
            .chars()
            .map(|c| match c {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Unrecognized Card"),
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let order = determine_order_p2(&cards);
        let bid = bid.parse().unwrap();
        Self { order, cards, bid }
    }
}

fn determine_order_p2(cards: &Cards) -> u64 {
    let mut counts = [0u8; 15];
    for card in cards {
        counts[*card as usize] += 1;
    }
    let jokers = counts[1];
    counts[1] = 0;
    counts.sort();
    counts[14] += jokers;
    match counts
        .into_iter()
        .rev()
        .take(2)
        .collect::<Vec<_>>()
        .as_slice()
    {
        [5, 0] => 6, // 5 of a Kind
        [4, 1] => 5, // 4 of a Kind
        [3, 2] => 4, // Full House
        [3, 1] => 3, // 3 of a Kind
        [2, 2] => 2, // Two Pairs
        [2, 1] => 1, // 2 of a Kind
        [1, 1] => 0, // High Card
        a => panic!("Unknown hand type: {a:?}"),
    }
}

fn day07_p1(deal: &str) -> u64 {
    let mut hands: Vec<Hand> = deal.lines().map(Hand::from).collect();
    hands.sort();
    let length = 1 + hands.len() as u64;
    hands
        .into_iter()
        .zip(1..length)
        .map(|(hand, rank)| rank * hand.bid)
        .sum()
}

fn day07_p2(deal: &str) -> u64 {
    let mut hands: Vec<Hand> = deal.lines().map(Hand::from_p2).collect();
    hands.sort();
    let length = 1 + hands.len() as u64;
    hands
        .into_iter()
        .zip(1..length)
        .map(|(hand, rank)| rank * hand.bid)
        .sum()
}

pub fn run_day07_p1() -> u64 {
    let filename = "data/day_07.txt";
    let deal = read_to_string(filename).unwrap();
    day07_p1(&deal)
}

pub fn run_day07_p2() -> u64 {
    let filename = "data/day_07.txt";
    let deal = read_to_string(filename).unwrap();
    day07_p2(&deal)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let sol = run_day07_p1();
        println!("Day 7 part 1 solution is: {sol}");
    } else {
        let sol = run_day07_p2();
        println!("Day 7 part 2 solution is: {sol}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483";

    #[test]
    fn test_day07_p1_example() {
        assert_eq!(day07_p1(EXAMPLE), 6440)
    }

    #[test]
    fn test_day07_p2_example() {
        assert_eq!(day07_p2(EXAMPLE), 5905)
    }

    #[test]
    fn test_day07_p1() {
        assert_eq!(run_day07_p1(), 252052080)
    }

    #[test]
    fn test_day07_p2() {
        assert_eq!(run_day07_p2(), 252898370)
    }
}
