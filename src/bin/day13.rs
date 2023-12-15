//! Day 13: Point of Incidence
#![allow(clippy::needless_range_loop)]
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn day13_p1_single(chart: &[&[u8]]) -> u64 {
    let height = chart.len();
    let width = chart[0].len();

    // find horizontal lines of symmetry (matching rows)
    'outer: for jj in 0..(height - 1) {
        for offset in 0..usize::min(jj + 1, height - 1 - jj) {
            for ii in 0..width {
                if chart[jj - offset][ii] != chart[jj + 1 + offset][ii] {
                    continue 'outer;
                }
            }
        }
        return ((jj + 1) * 100) as u64;
    }

    // find vertical lines of symmetry (matching columns)
    'outer2: for jj in 0..(width - 1) {
        for offset in 0..usize::min(jj + 1, width - 1 - jj) {
            for ii in 0..height {
                if chart[ii][jj - offset] != chart[ii][jj + 1 + offset] {
                    continue 'outer2;
                }
            }
        }
        return (jj + 1) as u64;
    }
    0
}

fn day13_p2_single(chart: &[&[u8]]) -> u64 {
    let height = chart.len();
    let width = chart[0].len();

    // find horizontal lines of symmetry (matching rows)
    'outer: for jj in 0..(height - 1) {
        let mut smudge_count = 0;
        for offset in 0..usize::min(jj + 1, height - 1 - jj) {
            for ii in 0..width {
                if chart[jj - offset][ii] != chart[jj + 1 + offset][ii] {
                    smudge_count += 1;
                    if smudge_count > 1 {
                        continue 'outer;
                    }
                }
            }
        }
        if smudge_count == 1 {
            return ((jj + 1) * 100) as u64;
        }
    }

    // find vertical lines of symmetry (matching columns)
    'outer2: for jj in 0..(width - 1) {
        let mut smudge_count = 0;
        for offset in 0..usize::min(jj + 1, width - 1 - jj) {
            for ii in 0..height {
                if chart[ii][jj - offset] != chart[ii][jj + 1 + offset] {
                    smudge_count += 1;
                    if smudge_count > 1 {
                        continue 'outer2;
                    }
                }
            }
        }
        if smudge_count == 1 {
            return (jj + 1) as u64;
        }
    }
    0
}

fn day13_p1(data: &str) -> u64 {
    data.split("\n\n")
        .map(|chart| {
            let chart: Vec<&[u8]> = chart.lines().map(|line| line.as_bytes()).collect();
            day13_p1_single(&chart)
        })
        .sum()
}

fn day13_p2(data: &str) -> u64 {
    data.split("\n\n")
        .map(|chart| {
            let chart: Vec<&[u8]> = chart.lines().map(|line| line.as_bytes()).collect();
            day13_p2_single(&chart)
        })
        .sum()
}

pub fn run_day13_p1() -> u64 {
    let filename = "data/day_13.txt";
    let data = read_to_string(filename).unwrap();
    day13_p1(&data)
}

pub fn run_day13_p2() -> u64 {
    let filename = "data/day_13.txt";
    let data = read_to_string(filename).unwrap();
    day13_p2(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let now = Instant::now();
        let sol = run_day13_p1();
        let elapsed = now.elapsed().as_secs();
        println!("Day 13 part 1 solution is: {sol} in {elapsed} seconds");
    } else {
        let now = Instant::now();
        let sol = run_day13_p2();
        let elapsed = now.elapsed().as_secs();
        println!("Day 13 part 2 solution is: {sol} in {elapsed} seconds");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        #.##..##.\n\
        ..#.##.#.\n\
        ##......#\n\
        ##......#\n\
        ..#.##.#.\n\
        ..##..##.\n\
        #.#.##.#.\n\
        \n\
        #...##..#\n\
        #....#..#\n\
        ..##..###\n\
        #####.##.\n\
        #####.##.\n\
        ..##..###\n\
        #....#..#";

    #[test]
    fn test_day13_p1_example() {
        assert_eq!(day13_p1(EXAMPLE), 405);
    }

    #[test]
    fn test_day13_p2_example() {
        assert_eq!(day13_p2(EXAMPLE), 400)
    }

    #[test]
    fn test_day13_p1() {
        assert_eq!(run_day13_p1(), 31739);
    }

    #[test]
    fn test_day13_p2() {
        assert_eq!(run_day13_p2(), 31539);
    }
}
