//! Day 1: Trebuchet?!
use std::fs::read_to_string;

/// Implements the main line-by-line logic of Day 1 part 1
fn day01_p1_iter<'a>(items: impl Iterator<Item = &'a str> + 'a) -> impl Iterator<Item = u32> + 'a {
    items
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<_>>()
        })
        .map(|line| {
            format!("{}{}", line.first().unwrap(), line.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
}

/// The goal of Day 1 part 1 is to extract the first and last digit in a string into a number.
/// The sum of numbers for all given strings is the answer.
/// For example
///    - "a1b2c3d4e5f" -> 15
///    - "hgre7njke" -> 77
/// Note that 7 in the last example was both the first and last digit.
pub fn day01_p1() -> u32 {
    let filename = "data/day_01.txt";
    day01_p1_iter(read_to_string(filename).unwrap().lines()).sum()
}

/// Implements the main line-by-line logic of Day 1 part 2
fn day01_p2_iter<'a>(items: impl Iterator<Item = &'a str> + 'a) -> impl Iterator<Item = u32> + 'a {
    items
        .map(|line| {
            line.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "4")
                .replace("five", "5e")
                .replace("six", "6")
                .replace("seven", "7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .replace("zero", "0o")
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<_>>()
        })
        .map(|line| {
            format!("{}{}", line.first().unwrap(), line.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
}

/// The objective of part 2 is a variation on part 1 of Day 1 where numbers spelled
/// out are also treated as digits for the sake of extracting the first and last digit.
/// "xtwone3four" -> 24 (notice the overlap in two and one)
/// "zoneight234" -> 14
pub fn day01_p2() -> u32 {
    let filename = "data/day_01.txt";
    day01_p2_iter(read_to_string(filename).unwrap().lines()).sum()
}

use std::env;
fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let sol = day01_p1();
        println!("Day 1 part 1 solution is: {sol}");
    } else {
        let sol = day01_p2();
        println!("Day 1 part 2 solution is: {sol}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day01_p1_example() {
        let example = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        assert_eq!(
            day01_p1_iter(example.into_iter()).collect::<Vec<_>>(),
            &[12, 38, 15, 77]
        )
    }

    #[test]
    fn test_day01_p1() {
        assert_eq!(day01_p1(), 55130);
    }

    #[test]
    fn test_day01_p2_example() {
        let example = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        assert_eq!(
            day01_p2_iter(example.into_iter()).collect::<Vec<_>>(),
            &[29, 83, 13, 24, 42, 14, 76]
        )
    }

    #[test]
    fn test_day01_p2() {
        assert_eq!(day01_p2(), 54985);
    }
}
