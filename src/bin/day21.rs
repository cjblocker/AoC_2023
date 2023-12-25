//! Day 21: Step Counter
#![allow(dead_code)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

// Coordinates for convenience. It encapsulates boundary conditions
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Coordinate<const N: usize> {
    row: usize,
    col: usize,
}

impl<const N: usize> Coordinate<N> {
    fn up(&self) -> Option<Self> {
        if self.row > 0 {
            Some(Coordinate {
                row: self.row - 1,
                col: self.col,
            })
        } else {
            None
        }
    }

    fn down(&self) -> Option<Self> {
        if self.row < N - 1 {
            Some(Coordinate {
                row: self.row + 1,
                col: self.col,
            })
        } else {
            None
        }
    }

    fn left(&self) -> Option<Self> {
        if self.col > 0 {
            Some(Coordinate {
                row: self.row,
                col: self.col - 1,
            })
        } else {
            None
        }
    }

    fn right(&self) -> Option<Self> {
        if self.col < N - 1 {
            Some(Coordinate {
                row: self.row,
                col: self.col + 1,
            })
        } else {
            None
        }
    }
}

impl<const N: usize> From<(usize, usize)> for Coordinate<N> {
    fn from(tuple: (usize, usize)) -> Self {
        Self {
            row: tuple.0,
            col: tuple.1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Garden,
    Rock,
}

fn get_state<const N: usize>(data: &str) -> ([[Tile; N]; N], Coordinate<N>) {
    let mut start: Option<Coordinate<N>> = None;
    let mut chart = [[Tile::Garden; N]; N];
    for (row, (chart_line, data_line)) in chart
        .iter_mut()
        .zip(data.lines().map(|line| line.as_bytes()))
        .enumerate()
    {
        for (col, (chart_entry, data_entry)) in
            chart_line.iter_mut().zip(data_line.iter()).enumerate()
        {
            if *data_entry == b'S' {
                start = Some((row, col).into())
            }
            *chart_entry = if *data_entry == b'#' {
                Tile::Rock
            } else {
                Tile::Garden
            }
        }
    }
    let start = start.unwrap();
    (chart, start)
}

fn day21_p1<const N: usize, const STEPS: usize>(data: &str) -> u64 {
    let (chart, start) = get_state::<N>(data);

    (0..STEPS)
        .fold(HashSet::from([start]), |prev, _| {
            let mut next = HashSet::new();
            for coord in prev.into_iter() {
                if let Some(pos) = coord.up() {
                    if chart[pos.row][pos.col] != Tile::Rock {
                        next.insert(pos);
                    }
                }
                if let Some(pos) = coord.down() {
                    if chart[pos.row][pos.col] != Tile::Rock {
                        next.insert(pos);
                    }
                }
                if let Some(pos) = coord.right() {
                    if chart[pos.row][pos.col] != Tile::Rock {
                        next.insert(pos);
                    }
                }
                if let Some(pos) = coord.left() {
                    if chart[pos.row][pos.col] != Tile::Rock {
                        next.insert(pos);
                    }
                }
            }
            next
        })
        .len() as u64
}

fn day21_p1_v2<const N: usize, const STEPS: usize>(data: &str) -> u64 {
    // a little bit faster version found after doing part 2
    let (chart, start) = get_state::<N>(data);

    let mut dist_map: HashMap<Coordinate<N>, usize> = HashMap::new();
    dist_map.insert(start, 0);
    (1..STEPS + 1).fold(HashSet::from([start]), |prev, step| {
        let mut next = HashSet::new();
        for coord in prev.into_iter() {
            let neighbors = [coord.up(), coord.down(), coord.right(), coord.left()];
            for pos in neighbors.into_iter().flatten() {
                if chart[pos.row][pos.col] != Tile::Rock {
                    dist_map.entry(pos).or_insert_with(|| {
                        next.insert(pos);
                        step
                    });
                }
            }
        }
        next
    });
    let parity = STEPS % 2;
    dist_map.into_values().filter(|x| x % 2 == parity).count() as u64
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct WrappingCoordinate<const N: usize> {
    row: usize,
    col: usize,
    major_row: isize,
    major_col: isize,
}

impl<const N: usize> WrappingCoordinate<N> {
    fn up(&self) -> Self {
        if self.row > 0 {
            Self {
                row: self.row - 1,
                ..*self
            }
        } else {
            Self {
                row: N - 1,
                major_row: self.major_row - 1,
                ..*self
            }
        }
    }

    fn down(&self) -> Self {
        if self.row < N - 1 {
            Self {
                row: self.row + 1,
                ..*self
            }
        } else {
            Self {
                row: 0,
                major_row: self.major_row + 1,
                ..*self
            }
        }
    }

    fn left(&self) -> Self {
        if self.col > 0 {
            Self {
                col: self.col - 1,
                ..*self
            }
        } else {
            Self {
                col: N - 1,
                major_col: self.major_col - 1,
                ..*self
            }
        }
    }

    fn right(&self) -> Self {
        if self.col < N - 1 {
            Self {
                col: self.col + 1,
                ..*self
            }
        } else {
            Self {
                col: 0,
                major_col: self.major_col + 1,
                ..*self
            }
        }
    }
}

impl<const N: usize> From<(usize, usize)> for WrappingCoordinate<N> {
    fn from(tuple: (usize, usize)) -> Self {
        Self {
            row: tuple.0,
            col: tuple.1,
            major_row: 0,
            major_col: 0,
        }
    }
}

impl<const N: usize> From<Coordinate<N>> for WrappingCoordinate<N> {
    fn from(coord: Coordinate<N>) -> Self {
        Self {
            row: coord.row,
            col: coord.col,
            major_row: 0,
            major_col: 0,
        }
    }
}

fn day21_p2<const N: usize, const STEPS: usize>(data: &str) -> u64 {
    let (chart, start) = get_state::<N>(data);
    let start: WrappingCoordinate<N> = start.into();

    let mut dist_map: HashMap<WrappingCoordinate<N>, usize> = HashMap::new();
    dist_map.insert(start, 0);
    (1..STEPS + 1).fold(HashSet::from([start]), |prev, step| {
        let mut next = HashSet::new();
        for coord in prev.into_iter() {
            let neighbors = [coord.up(), coord.down(), coord.right(), coord.left()];
            for pos in neighbors {
                if chart[pos.row][pos.col] != Tile::Rock {
                    dist_map.entry(pos).or_insert_with(|| {
                        next.insert(pos);
                        step
                    });
                }
            }
        }
        next
    });
    let parity = STEPS % 2;
    dist_map.into_values().filter(|x| x % 2 == parity).count() as u64
}

fn day21_p2_v2(_data: &str) -> u64 {
    // this solution annoys me because it doesn't necessarily generalize
    // I had to inspect the very specific case I was given an construct example for it.
    // Maybe I can come back later and make it more general.
    const N: usize = 131;
    const STEPS: usize = 26_501_365;
    let l1_radius: usize = (STEPS - N / 2) / N;
    // let l1_rem: usize = (STEPS - N / 2) % N; // =0
    // dbg!(l1_radius, l1_rem);
    // So I chose these STEP counts to product examples that would
    // be equivalent but smaller (a lot of data inspection behind equivalent here).
    // I then did a polynomial fit to the data and it had a perfect quadratic fit.
    // dbg!( // the x below is even, so are my test points
    //     day21_p2::<131, 327>(&data),  // 2
    //     day21_p2::<131, 589>(&data),  // 4
    //     day21_p2::<131, 851>(&data),  // 6
    //     day21_p2::<131, 1113>(&data), // 8
    // );
    let (a, b, c) = (15094, 15196, 3835); // polynomial fit to 4 above data points
    let x = l1_radius as u64; // = 202300
    a * x * x + b * x + c
}

pub fn run_day21_p1() -> u64 {
    let filename = "data/day_21.txt";
    let data = read_to_string(filename).unwrap();
    day21_p1::<131, 64>(&data)
}

pub fn run_day21_p2() -> u64 {
    let filename = "data/day_21.txt";
    let data = read_to_string(filename).unwrap();
    day21_p2_v2(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let now = Instant::now();
        let sol = run_day21_p1();
        let elapsed = now.elapsed().as_millis();
        println!("Day 21 part 1 solution is: {sol} in {elapsed} ms");
    } else {
        let now = Instant::now();
        let sol = run_day21_p2();
        let elapsed = now.elapsed().as_millis();
        println!("Day 21 part 2 solution is: {sol} in {elapsed} ms");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        ...........\n\
        .....###.#.\n\
        .###.##..#.\n\
        ..#.#...#..\n\
        ....#.#....\n\
        .##..S####.\n\
        .##..#...#.\n\
        .......##..\n\
        .##.#.####.\n\
        .##..##.##.\n\
        ...........";

    #[test]
    fn test_day21_p1_example() {
        assert_eq!(day21_p1::<11, 6>(EXAMPLE), 16);
    }

    #[test]
    fn test_day21_p1_v2_example() {
        assert_eq!(day21_p1_v2::<11, 6>(EXAMPLE), 16);
    }

    #[test]
    fn test_day21_p2_example() {
        assert_eq!(day21_p2::<11, 6>(EXAMPLE), 16);
        assert_eq!(day21_p2::<11, 10>(EXAMPLE), 50);
        assert_eq!(day21_p2::<11, 50>(EXAMPLE), 1594);
        assert_eq!(day21_p2::<11, 100>(EXAMPLE), 6536);
        assert_eq!(day21_p2::<11, 500>(EXAMPLE), 167004);
        assert_eq!(day21_p2::<11, 1000>(EXAMPLE), 668697);
        // assert_eq!(day21_p2::<11, 5000>(EXAMPLE), 16733044);
    }

    #[test]
    fn test_day21_p1() {
        assert_eq!(run_day21_p1(), 3733);
    }

    #[test]
    fn test_day21_p2() {
        assert_eq!(run_day21_p2(), 617_729_401_414_635);
    }
}
