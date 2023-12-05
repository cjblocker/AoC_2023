use std::fs::read_to_string;

/// Each card has winning numbers and given numbers. The points of a card is the
/// number of given numbers that are in the winning numbers.
/// Here we return the sum of all points.
fn day04_p1(cards: &str) -> u32 {
    // parse the data in an iterator of number of winning numbers for each card
    cards
        .lines()
        .map(|line| {
            // we don't need the card number at all
            let (_, card_data) = line.split_once(": ").unwrap();
            let (winners, given) = card_data.split_once(" | ").unwrap();
            let winners: Vec<u32> = winners
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            let win_count = given
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .filter(|x| winners.contains(x))
                .count();
            if win_count > 0 {
                1 << (win_count - 1) // 2^(win_count - 1)
            } else {
                0
            }
        })
        .sum()
}

pub fn run_day04_p1() -> u32 {
    let filename = "data/day_04.txt";
    let cards = read_to_string(filename).unwrap();
    day04_p1(&cards)
}

/// Instead of points, the number of matching numbers on each card indicates
/// the number of copies of following cards you receive. So 4 winning numbers on
/// card 1 grants you another copy of cards 2,3,4,5 in addition to the original ones you had.
fn day04_p2(cards: &str) -> u32 {
    // parse the data in an iterator of number of winning numbers for each card
    let counts = cards.lines().map(|line| {
        // we don't need the card number at all
        let (_, card_data) = line.split_once(": ").unwrap();
        let (winners, given) = card_data.split_once(" | ").unwrap();
        let winners: Vec<u32> = winners
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let win_count = given
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .filter(|x| winners.contains(x))
            .count();
        win_count
    });

    // The value of a card is how many downstream cards it adds in total
    // Since cards only depend on cards ahead of them, we can iterate through
    // the cards backwards and assign each its "value" based on the "value" of
    // the cards it adds (+ 1 for the original card).
    let mut value = Vec::new();
    for count in counts.rev() {
        let cur_value: u32 = value.iter().rev().take(count).sum::<u32>() + 1;
        value.push(cur_value)
    }
    value.into_iter().sum()
}

pub fn run_day04_p2() -> u32 {
    let filename = "data/day_04.txt";
    let cards = read_to_string(filename).unwrap();
    day04_p2(&cards)
}

use std::env;
fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let sol = run_day04_p1();
        println!("Day 4 part 1 solution is: {sol}");
    } else {
        let sol = run_day04_p2();
        println!("Day 4 part 2 solution is: {sol}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_day04_p1_example() {
        assert_eq!(day04_p1(EXAMPLE), 13)
    }

    #[test]
    fn test_day04_p2_example() {
        assert_eq!(day04_p2(EXAMPLE), 30)
    }

    #[test]
    fn test_day04_p1() {
        assert_eq!(run_day04_p1(), 25571)
    }

    #[test]
    fn test_day04_p2() {
        assert_eq!(run_day04_p2(), 8805731)
    }
}
