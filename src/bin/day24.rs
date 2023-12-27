//! Day 24: Never Tell Me The Odds
// use rayon::prelude::*;
use std::env;
use std::fs::read_to_string;
use std::marker::PhantomData;
use std::time::Instant;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Position;
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Velocity;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Vector<Quantity> {
    x: u64,
    y: u64,
    z: u64,
    quantity: PhantomData<Quantity>,
}

impl<Quantity> From<&str> for Vector<Quantity> {
    fn from(data: &str) -> Self {
        let [x, y, z]: [u64; 3] = data
            .split(' ')
            .map(|t| t.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self {
            x,
            y,
            z,
            quantity: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Hailstone {
    pos: Vector<Position>,
    vel: Vector<Velocity>,
}

impl From<&str> for Hailstone {
    fn from(data: &str) -> Self {
        let (pos, vel) = data.split_once(" @ ").unwrap();
        Self {
            pos: pos.into(),
            vel: vel.into(),
        }
    }
}

impl Hailstone {
    fn intersection(&self, other: &Self) -> Option<[u64; 2]> {
        unimplemented!();
    }
}

fn day24_p1(data: &str, lower: u64, upper: u64) -> usize {
    let hailstones: Vec<Hailstone> = data.lines().map(Hailstone::from).collect();
    hailstones
        .iter()
        .enumerate()
        .map(|(ii, hailstone1)| {
            hailstones
                .iter()
                .skip(ii + 1)
                .filter(|hailstone2| {
                    hailstone1
                        .intersection(hailstone2)
                        .filter(|pos| pos.iter().all(|p| (p >= &lower) && (p <= &upper)))
                        .is_some()
                })
                .count()
        })
        .sum()
}

fn day24_p2(data: &str) -> u64 {
    0
}

pub fn run_day24_p1() -> usize {
    let filename = "data/day_24.txt";
    let data = read_to_string(filename).unwrap();
    day24_p1(&data, 200000000000000, 400000000000000)
}

pub fn run_day24_p2() -> u64 {
    // super slow
    let filename = "data/day_24.txt";
    let data = read_to_string(filename).unwrap();
    day24_p2(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let now = Instant::now();
        let sol = run_day24_p1();
        let elapsed = now.elapsed().as_millis();
        println!("Day 24 part 1 solution is: {sol} in {elapsed} ms");
    } else {
        let now = Instant::now();
        let sol = run_day24_p2();
        let elapsed = now.elapsed().as_millis();
        println!("Day 24 part 2 solution is: {sol} in {elapsed} ms");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        19, 13, 30 @ -2,  1, -2\n\
        18, 19, 22 @ -1, -1, -2\n\
        20, 25, 34 @ -2, -2, -4\n\
        12, 31, 28 @ -1, -2, -1\n\
        20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_day24_p1_example() {
        assert_eq!(day24_p1(EXAMPLE, 7, 27), 2);
    }

    #[test]
    #[ignore]
    fn test_day24_p2_example() {
        assert_eq!(day24_p2(EXAMPLE), 0);
    }

    #[test]
    #[ignore]
    fn test_day24_p1() {
        assert_eq!(run_day24_p1(), 0);
    }

    #[test]
    #[ignore]
    fn test_day24_p2() {
        // takes almost 2 minutes
        assert_eq!(run_day24_p2(), 0);
    }
}
