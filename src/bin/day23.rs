//! Day 23: A Long Walk
use rayon::prelude::*;
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
    Path,
    Forest,
    UpSlope,
    DownSlope,
    RightSlope,
    LeftSlope,
}

fn get_state<const N: usize>(data: &str) -> ([[Tile; N]; N], Coordinate<N>) {
    let mut chart = [[Tile::Forest; N]; N];
    for (chart_line, data_line) in chart
        .iter_mut()
        .zip(data.lines().map(|line| line.as_bytes()))
    {
        for (chart_entry, data_entry) in chart_line.iter_mut().zip(data_line.iter()) {
            *chart_entry = match *data_entry {
                b'.' => Tile::Path,
                b'#' => Tile::Forest,
                b'^' => Tile::UpSlope,
                b'v' => Tile::DownSlope,
                b'>' => Tile::RightSlope,
                b'<' => Tile::LeftSlope,
                c => panic!("Unrecognized {:?}", c),
            };
        }
    }
    let start = Coordinate {
        row: 0,
        col: chart[0]
            .iter()
            .position(|&tile| tile == Tile::Path)
            .unwrap(),
    };
    (chart, start)
}

fn find_longest_path<const N: usize>(
    chart: &[[Tile; N]; N],
    pos: Coordinate<N>,
    mut used: HashSet<Coordinate<N>>,
) -> Option<u32> {
    let mut dist: u32 = 0;
    let mut next_pos: Vec<Coordinate<N>> = vec![pos];
    let mut pos = pos;
    let mut last_pos;
    while next_pos.len() == 1 {
        last_pos = pos;
        pos = next_pos.pop().unwrap();
        dist += 1;
        if pos.row == N - 1 {
            return Some(dist);
        }
        next_pos = [
            (pos.up().unwrap(), Tile::UpSlope),
            (pos.down().unwrap(), Tile::DownSlope),
            (pos.right().unwrap(), Tile::RightSlope),
            (pos.left().unwrap(), Tile::LeftSlope),
        ]
        .into_iter()
        .filter(|&(nx_pos, ttype)| {
            (chart[nx_pos.row][nx_pos.col] == Tile::Path || chart[nx_pos.row][nx_pos.col] == ttype)
                && (nx_pos != last_pos)
                && (!used.contains(&nx_pos))
        })
        .map(|(nx_pos, _)| nx_pos)
        .collect();
    }
    if next_pos.is_empty() {
        return None;
    }
    used.insert(pos);
    dist += next_pos
        .into_iter()
        .map(|nx_pos| find_longest_path(chart, nx_pos, used.clone()))
        .flatten()
        .max()?;
    Some(dist)
}

fn day23_p1<const N: usize>(data: &str) -> u32 {
    let (chart, start) = get_state::<N>(data);
    let pos = start.down().unwrap();
    let mut used = HashSet::new();
    used.insert(start);
    find_longest_path(&chart, pos, used).unwrap()
}

fn find_longest_path_p2<const N: usize>(
    chart: &[[Tile; N]; N],
    pos: Coordinate<N>,
    mut used: HashSet<Coordinate<N>>,
) -> Option<u32> {
    let mut dist: u32 = 0;
    let mut next_pos: Vec<Coordinate<N>> = vec![pos];
    let mut pos = pos;
    let mut last_pos;
    while next_pos.len() == 1 {
        last_pos = pos;
        pos = next_pos.pop().unwrap();
        dist += 1;
        if pos.row == N - 1 {
            return Some(dist);
        }
        next_pos = [
            pos.up().unwrap(),
            pos.down().unwrap(),
            pos.right().unwrap(),
            pos.left().unwrap(),
        ]
        .into_iter()
        .filter(|&nx_pos| {
            (chart[nx_pos.row][nx_pos.col] != Tile::Forest)
                && (nx_pos != last_pos)
                && (!used.contains(&nx_pos))
        })
        .collect();
    }
    if next_pos.is_empty() {
        return None;
    }
    used.insert(pos);
    dist += next_pos
        .into_par_iter()
        .map(|nx_pos| find_longest_path_p2(chart, nx_pos, used.clone()))
        .flatten()
        .max()?;
    Some(dist)
}

fn day23_p2<const N: usize>(data: &str) -> u32 {
    let (chart, start) = get_state::<N>(data);
    let pos = start.down().unwrap();
    let mut used = HashSet::new();
    used.insert(start);
    find_longest_path_p2(&chart, pos, used).unwrap()
}

pub fn run_day23_p1() -> u32 {
    let filename = "data/day_23.txt";
    let data = read_to_string(filename).unwrap();
    day23_p1::<141>(&data)
}

pub fn run_day23_p2() -> u32 {
    // super slow
    let filename = "data/day_23.txt";
    let data = read_to_string(filename).unwrap();
    day23_p2::<141>(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let now = Instant::now();
        let sol = run_day23_p1();
        let elapsed = now.elapsed().as_millis();
        println!("Day 23 part 1 solution is: {sol} in {elapsed} ms");
    } else {
        let now = Instant::now();
        let sol = run_day23_p2();
        let elapsed = now.elapsed().as_millis();
        println!("Day 23 part 2 solution is: {sol} in {elapsed} ms");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        #.#####################\n\
        #.......#########...###\n\
        #######.#########.#.###\n\
        ###.....#.>.>.###.#.###\n\
        ###v#####.#v#.###.#.###\n\
        ###.>...#.#.#.....#...#\n\
        ###v###.#.#.#########.#\n\
        ###...#.#.#.......#...#\n\
        #####.#.#.#######.#.###\n\
        #.....#.#.#.......#...#\n\
        #.#####.#.#.#########v#\n\
        #.#...#...#...###...>.#\n\
        #.#.#v#######v###.###v#\n\
        #...#.>.#...>.>.#.###.#\n\
        #####v#.#.###v#.#.###.#\n\
        #.....#...#...#.#.#...#\n\
        #.#########.###.#.#.###\n\
        #...###...#...#...#.###\n\
        ###.###.#.###v#####v###\n\
        #...#...#.#.>.>.#.>.###\n\
        #.###.###.#.###.#.#v###\n\
        #.....###...###...#...#\n\
        #####################.#";

    #[test]
    fn test_day23_p1_example() {
        assert_eq!(day23_p1::<23>(EXAMPLE), 94);
    }

    #[test]
    fn test_day23_p2_example() {
        assert_eq!(day23_p2::<23>(EXAMPLE), 154);
    }

    #[test]
    fn test_day23_p1() {
        assert_eq!(run_day23_p1(), 2070);
    }

    #[test]
    #[ignore]
    fn test_day23_p2() {
        // takes almost 2 minutes
        assert_eq!(run_day23_p2(), 6498);
    }
}
