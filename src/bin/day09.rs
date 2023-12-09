//! Day 9: Mirage Maintenance
use std::env;
use std::fs::read_to_string;

fn extrapolate_next(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|&x| x == 0) {
        return 0;
    }
    numbers[numbers.len() - 1]
        + extrapolate_next(
            &numbers
                .windows(2)
                .map(|d| d[1] - d[0])
                .collect::<Vec<i64>>(),
        )
}

fn day09_p1(data: &str) -> i64 {
    // basically, we are doing polynomial extrapolation.
    // Is it better to take the approach described, or use matrix inversion methods?
    // It's nice they are all integers, and the provided method has a nice recursive structure
    // (Yeah, it ended up being very quick day with recursion).
    data.lines()
        .map(|line| {
            extrapolate_next(
                &line
                    .split(' ')
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .sum()
}

fn extrapolate_prev(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|&x| x == 0) {
        return 0;
    }
    numbers[0]
        - extrapolate_prev(
            &numbers
                .windows(2)
                .map(|d| d[1] - d[0])
                .collect::<Vec<i64>>(),
        )
}

fn day09_p2(data: &str) -> i64 {
    data.lines()
        .map(|line| {
            extrapolate_prev(
                &line
                    .split(' ')
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .sum()
}

pub fn run_day09_p1() -> i64 {
    let filename = "data/day_09.txt";
    let data = read_to_string(filename).unwrap();
    day09_p1(&data)
}

pub fn run_day09_p2() -> i64 {
    let filename = "data/day_09.txt";
    let data = read_to_string(filename).unwrap();
    day09_p2(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let sol = run_day09_p1();
        println!("Day 9 part 1 solution is: {sol}");
    } else {
        let sol = run_day09_p2();
        println!("Day 9 part 2 solution is: {sol}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        0 3 6 9 12 15\n\
        1 3 6 10 15 21\n\
        10 13 16 21 30 45";

    #[test]
    fn test_day09_p1_example() {
        assert_eq!(day09_p1(EXAMPLE), 114)
    }

    #[test]
    fn test_day09_p2_example() {
        assert_eq!(day09_p2(EXAMPLE), 2)
    }

    #[test]
    fn test_day09_p1() {
        assert_eq!(run_day09_p1(), 1798691765)
    }

    #[test]
    fn test_day09_p2() {
        assert_eq!(run_day09_p2(), 1104)
    }
}
