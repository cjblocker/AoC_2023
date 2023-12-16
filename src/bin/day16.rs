//! Day 16: The Floor Will Be Lava
use rayon::prelude::*;
use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
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

    fn down(&self, height: usize) -> Option<Self> {
        if self.row < height - 1 {
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

    fn right(&self, width: usize) -> Option<Self> {
        if self.col < width - 1 {
            Some(Coordinate {
                row: self.row,
                col: self.col + 1,
            })
        } else {
            None
        }
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(tuple: (usize, usize)) -> Self {
        Self {
            row: tuple.0,
            col: tuple.1,
        }
    }
}

fn run_beam_sim(chart: &[&[u8]], initial: (Coordinate, Direction)) -> usize {
    let height = chart.len();
    let width = chart[0].len();
    // these are the tiles that we need to process (potential wave fronts)
    let mut visit_stack = vec![initial];
    // these are all wave fronts we have already seen (to avoid loops)
    let mut visited = HashSet::<(Coordinate, Direction)>::new();
    // these are all of the unique tiles we've visited
    let mut energized = HashSet::<Coordinate>::new();
    while let Some((tile, dir)) = visit_stack.pop() {
        energized.insert(tile);
        if visited.contains(&(tile, dir)) {
            // we should never run into a collision on a '.' space
            // but I don't optimize for that.
            continue;
        }
        visited.insert((tile, dir));
        let component = chart[tile.row][tile.col];
        match (component, dir) {
            (b'.' | b'|', Up) | (b'/', Right) | (b'\\', Left) => {
                if let Some(tile) = tile.up() {
                    visit_stack.push((tile, Up));
                }
            }
            (b'.' | b'|', Down) | (b'/', Left) | (b'\\', Right) => {
                if let Some(tile) = tile.down(height) {
                    visit_stack.push((tile, Down));
                }
            }
            (b'.' | b'-', Left) | (b'/', Down) | (b'\\', Up) => {
                if let Some(tile) = tile.left() {
                    visit_stack.push((tile, Left));
                }
            }
            (b'.' | b'-', Right) | (b'/', Up) | (b'\\', Down) => {
                if let Some(tile) = tile.right(width) {
                    visit_stack.push((tile, Right));
                }
            }
            (b'|', Left | Right) => {
                if let Some(tile) = tile.up() {
                    visit_stack.push((tile, Up));
                }
                if let Some(tile) = tile.down(height) {
                    visit_stack.push((tile, Down));
                }
            }
            (b'-', Up | Down) => {
                if let Some(tile) = tile.left() {
                    visit_stack.push((tile, Left));
                }
                if let Some(tile) = tile.right(width) {
                    visit_stack.push((tile, Right));
                }
            }
            _ => unreachable!(),
        }
    }
    // for row in 0..height {
    //     for col in 0..width {
    //         let c = if energized.contains(&(row, col).into()) {
    //             '#'
    //         } else {
    //             '.'
    //         };
    //         print!("{c}");
    //     }
    //     println!();
    // }
    energized.len()
}

fn day16_p1(data: &str) -> usize {
    let chart = data.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    run_beam_sim(&chart, ((0, 0).into(), Right))
}

fn day16_p2(data: &str) -> usize {
    // this runs in about 4 seconds without parallelization, but that's
    // a tad annoying for unit testing so I threw in rayon to speed it up 8x
    let chart = data.lines().map(|line| line.as_bytes()).collect::<Vec<_>>();
    let height = chart.len();
    let width = chart[0].len();
    (0..width)
        .into_par_iter()
        .map(|col| ((0, col), Down))
        .chain(
            (0..width)
                .into_par_iter()
                .map(|col| ((height - 1, col), Up))
                .chain(
                    (0..height)
                        .into_par_iter()
                        .map(|row| ((row, 0), Right))
                        .chain(
                            (0..height)
                                .into_par_iter()
                                .map(|row| ((row, width - 1), Left)),
                        ),
                ),
        )
        .map(|(pos, dir)| run_beam_sim(&chart, (pos.into(), dir)))
        .max()
        .unwrap()
}

pub fn run_day16_p1() -> usize {
    let filename = "data/day_16.txt";
    let data = read_to_string(filename).unwrap();
    day16_p1(&data)
}

pub fn run_day16_p2() -> usize {
    let filename = "data/day_16.txt";
    let data = read_to_string(filename).unwrap();
    day16_p2(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let now = Instant::now();
        let sol = run_day16_p1();
        let elapsed = now.elapsed().as_millis();
        println!("Day 16 part 1 solution is: {sol} in {elapsed} ms");
    } else {
        let now = Instant::now();
        let sol = run_day16_p2();
        let elapsed = now.elapsed().as_millis();
        println!("Day 16 part 2 solution is: {sol} in {elapsed} ms");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_day16_p1_example() {
        assert_eq!(day16_p1(EXAMPLE), 46);
    }

    #[test]
    fn test_day16_p2_example() {
        assert_eq!(day16_p2(EXAMPLE), 51)
    }

    #[test]
    fn test_day16_p1() {
        assert_eq!(run_day16_p1(), 7210);
    }

    #[test]
    fn test_day16_p2() {
        assert_eq!(run_day16_p2(), 7673);
    }
}
