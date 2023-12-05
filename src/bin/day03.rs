#![allow(clippy::needless_range_loop)] // I find the take(.).skip(.) syntax unclear
use std::collections::HashMap;
use std::fs::read_to_string;

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && (c != '.')
}

/// Find all numbers in the schematic that are adjacent to a symbol including diagonally
fn day03_p1(schematic: &str) -> u32 {
    let schematic: Vec<Vec<char>> = schematic
        .lines()
        .map(|line| {
            let mut line: Vec<char> = line.chars().collect();
            line.push('.'); // avoid line-end boundary condition
            line
        })
        .collect();
    // dbg!(&schematic);
    let mut sum = 0;
    // we loop through the schematic a character at a time
    // to build a number and then check if its valid (i.e. neighboring a symbol).
    let mut num = "".to_string();
    for (ii, line) in schematic.iter().enumerate() {
        for (jj, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                // we are in the process of forming number
                num.push(*c);
            } else if !num.is_empty() {
                // we just finished forming a number
                // check if its valid
                let mut valid = false;
                let above = ii.saturating_sub(1);
                let below = usize::min(ii + 1, schematic.len() - 1);
                let right = jj; // the only index that is definitely valid
                let left = jj.saturating_sub(num.len() + 1);
                // search box for symbols (includes `num` digits for simplicity)
                for kk in above..=below {
                    for ll in left..=right {
                        if is_symbol(schematic[kk][ll]) {
                            valid = true;
                            break;
                        }
                    }
                }
                if valid {
                    sum += num.parse::<u32>().unwrap();
                }

                // ready to start new number
                num.clear();
            }
        }
        // We made every line end in a period
        // so that we are never finishing a number by newline
        assert!(num.is_empty());
    }
    sum
}

/// A gear is a '*' with exactly two numbers by it
/// find all gears, multiply their two numbers, and add them up
fn day03_p2(schematic: &str) -> u32 {
    let schematic: Vec<Vec<char>> = schematic
        .lines()
        .map(|line| {
            let mut line: Vec<char> = line.chars().collect();
            line.push('.'); // avoid line-end boundary condition
            line
        })
        .collect();
    let mut possible_gears = HashMap::new();
    // we loop through the schematic a character at a time
    // to build a number. If it has a neighboring '*', we append it to that '*'s entry in a map
    let mut num = "".to_string();
    for (ii, line) in schematic.iter().enumerate() {
        for (jj, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                // we are in the process of forming number
                num.push(*c);
            } else if !num.is_empty() {
                // we just finished forming a number
                let above = ii.saturating_sub(1);
                let below = usize::min(ii + 1, schematic.len() - 1);
                let right = jj; // the only index that is definitely valid
                let left = jj.saturating_sub(num.len() + 1);
                // search box for '*' (includes `num` digits for simplicity)
                for kk in above..=below {
                    for ll in left..=right {
                        if schematic[kk][ll] == '*' {
                            let number = num.parse::<u32>().unwrap();
                            let entry = possible_gears.entry((kk, ll)).or_insert(Vec::<u32>::new());
                            (*entry).push(number);
                        }
                    }
                }
                // ready to start new number
                num.clear();
            }
        }
        // We made every line end in a period
        // so that we are never finishing a number by newline
        assert!(num.is_empty());
    }
    // now iterate over all possible gears to find true gears
    possible_gears
        .into_values()
        .filter(|x| x.len() == 2)
        .map(|x| x.first().unwrap() * x.last().unwrap())
        .sum()
}

pub fn run_day03_p1() -> u32 {
    let filename = "data/day_03.txt";
    let schematic = read_to_string(filename).unwrap();
    day03_p1(&schematic)
}

pub fn run_day03_p2() -> u32 {
    let filename = "data/day_03.txt";
    let schematic = read_to_string(filename).unwrap();
    day03_p2(&schematic)
}

use std::env;
fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let sol = run_day03_p1();
        println!("Day 3 part 1 solution is: {sol}");
    } else {
        let sol = run_day03_p2();
        println!("Day 3 part 2 solution is: {sol}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..";

    #[test]
    fn test_day03_p1_example() {
        assert_eq!(day03_p1(EXAMPLE), 4361)
    }

    #[test]
    fn test_day03_p2_example() {
        assert_eq!(day03_p2(EXAMPLE), 467835)
    }

    #[test]
    fn test_day03_p1() {
        assert_eq!(run_day03_p1(), 557705)
    }

    #[test]
    fn test_day03_p2() {
        assert_eq!(run_day03_p2(), 84266818)
    }
}
