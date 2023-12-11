use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

/// Find all numbers in the chart that are adjacent to a symbol including diagonally
fn day10_p1(chart: &str) -> u32 {
    let chart: Vec<Vec<char>> = chart
        .lines()
        .map(|line| {
            let mut line: Vec<char> = line.chars().collect();
            line.push('.'); // avoid line-end boundary condition
            line
        })
        .collect();

    let mut start = None;
    for (ii, line) in chart.iter().enumerate() {
        for (jj, c) in line.iter().enumerate() {
            if *c == 'S' {
                start = Some((ii, jj));
                break;
            }
        }
    }
    let start = start.unwrap();

    let mut pos = (start.0, start.1 + 1);
    let mut dir = Direction::East;
    let mut pipe = 'J';
    let mut count = 1;
    while pipe != 'S' {
        dir = match (pipe, dir) {
            ('-', Direction::East) => Direction::East,
            ('-', Direction::West) => Direction::West,
            ('|', Direction::North) => Direction::North,
            ('|', Direction::South) => Direction::South,
            ('L', Direction::South) => Direction::East,
            ('L', Direction::West) => Direction::North,
            ('J', Direction::South) => Direction::West,
            ('J', Direction::East) => Direction::North,
            ('7', Direction::North) => Direction::West,
            ('7', Direction::East) => Direction::South,
            ('F', Direction::North) => Direction::East,
            ('F', Direction::West) => Direction::South,
            _ => panic!("Error can't follow pipe: {:?},{:?}", pipe, dir),
        };
        match dir {
            Direction::North => pos.0 -= 1,
            Direction::South => pos.0 += 1,
            Direction::East => pos.1 += 1,
            Direction::West => pos.1 -= 1,
        };
        pipe = chart[pos.0][pos.1];
        count += 1;
    }
    dbg!(count);
    count / 2
}

/// A gear is a '*' with exactly two numbers by it
/// find all gears, multiply their two numbers, and add them up
fn day10_p2(chart: &str) -> u32 {
    0
}

pub fn run_day10_p1() -> u32 {
    let filename = "data/day_10.txt";
    let chart = read_to_string(filename).unwrap();
    day10_p1(&chart)
}

pub fn run_day10_p2() -> u32 {
    let filename = "data/day_10.txt";
    let chart = read_to_string(filename).unwrap();
    day10_p2(&chart)
}

use std::env;
fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let sol = run_day10_p1();
        println!("Day 10 part 1 solution is: {sol}");
    } else {
        let sol = run_day10_p2();
        println!("Day 10 part 2 solution is: {sol}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        7-F7-\n\
        .FJ|7\n\
        SJLL7\n\
        |F--J\n\
        LJ.LJ";

    #[test]
    fn test_day10_p1_example() {
        assert_eq!(day10_p1(EXAMPLE), 8)
    }

    // #[test]
    // fn test_day10_p2_example() {
    //     assert_eq!(day10_p2(EXAMPLE), 467835)
    // }

    #[test]
    fn test_day10_p1() {
        assert_eq!(run_day10_p1(), 6823)
    }

    // #[test]
    // fn test_day10_p2() {
    //     assert_eq!(run_day10_p2(), 84266818)
    // }
}
