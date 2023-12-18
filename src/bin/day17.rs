//! Day 17: Clumsy Crucible
// experimenting with const generics
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    Horizontal,
    Vertical,
}
use Direction::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Node<const N: usize> {
    cost: u16,
    position: Coordinate<N>,
    direction: Direction,
}

impl<const N: usize> Ord for Node<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.direction.cmp(&other.direction))
    }
}
impl<const N: usize> PartialOrd for Node<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

#[derive(Debug)]
struct DirState<const N: usize, Type> {
    horz: [[Type; N]; N],
    vert: [[Type; N]; N],
}

fn find_path<const N: usize, const MINSTEPS: usize, const MAXSTEPS: usize>(data: &str) -> u16 {
    let mut chart = [[0u16; N]; N];
    for (chart_line, data_line) in chart
        .iter_mut()
        .zip(data.lines().map(|line| line.as_bytes()))
    {
        for (chart_entry, data_entry) in chart_line.iter_mut().zip(data_line.iter()) {
            *chart_entry = (data_entry - b'0') as u16;
        }
    }
    let mut distance = DirState {
        horz: [[u16::MAX; N]; N],
        vert: [[u16::MAX; N]; N],
    };
    let start_indx: Coordinate<N> = (0, 0).into();
    let end_indx: Coordinate<N> = (N - 1, N - 1).into();
    distance.horz[start_indx.row][start_indx.col] = 0;
    distance.vert[start_indx.row][start_indx.col] = 0;
    let mut pqueue = BinaryHeap::new();

    // visit "nodes" adjacent to starting node
    // visit three below start
    let mut cur_indx = start_indx;
    let mut dist = 0;
    for step in 0..MAXSTEPS {
        cur_indx = if let Some(cur_indx) = cur_indx.down() {
            dist += chart[cur_indx.row][cur_indx.col];
            if step >= (MINSTEPS - 1) {
                distance.vert[cur_indx.row][cur_indx.col] = dist;
                pqueue.push(Node {
                    cost: dist,
                    position: cur_indx,
                    direction: Vertical,
                });
            }
            cur_indx
        } else {
            break;
        }
    }
    // visit three to the right of start
    let mut cur_indx = start_indx;
    let mut dist = 0;
    for step in 0..MAXSTEPS {
        cur_indx = if let Some(cur_indx) = cur_indx.right() {
            dist += chart[cur_indx.row][cur_indx.col];
            if step >= (MINSTEPS - 1) {
                distance.horz[cur_indx.row][cur_indx.col] = dist;
                pqueue.push(Node {
                    cost: dist,
                    position: cur_indx,
                    direction: Horizontal,
                });
            }
            cur_indx
        } else {
            break;
        }
    }

    // while the end node has not been visited (from both directions)
    while let Some(Node {
        cost: dist,
        position: indx,
        direction: dir,
    }) = pqueue.pop()
    {
        if indx == end_indx {
            return dist;
        }
        let old_dist = match dir {
            Horizontal => distance.horz[indx.row][indx.col],
            Vertical => distance.vert[indx.row][indx.col],
        };
        if dist != old_dist {
            // this node is out-of-date
            // dbg!(old_dist, dist, indx);
            assert!(old_dist < dist);
            continue;
        }
        match dir {
            Horizontal => {
                // visit three below indx
                let mut cur_indx = indx;
                let mut cur_dist = dist;
                for step in 0..MAXSTEPS {
                    cur_indx = if let Some(cur_indx) = cur_indx.down() {
                        cur_dist += chart[cur_indx.row][cur_indx.col];
                        if step >= (MINSTEPS - 1)
                            && cur_dist < distance.vert[cur_indx.row][cur_indx.col]
                        {
                            pqueue.push(Node {
                                cost: cur_dist,
                                position: cur_indx,
                                direction: Vertical,
                            });
                            distance.vert[cur_indx.row][cur_indx.col] = cur_dist;
                        }
                        cur_indx
                    } else {
                        break;
                    }
                }
                // visit three above indx
                let mut cur_indx = indx;
                let mut cur_dist = dist;
                for step in 0..MAXSTEPS {
                    cur_indx = if let Some(cur_indx) = cur_indx.up() {
                        cur_dist += chart[cur_indx.row][cur_indx.col];
                        if step >= (MINSTEPS - 1)
                            && cur_dist < distance.vert[cur_indx.row][cur_indx.col]
                        {
                            pqueue.push(Node {
                                cost: cur_dist,
                                position: cur_indx,
                                direction: Vertical,
                            });
                            distance.vert[cur_indx.row][cur_indx.col] = cur_dist;
                        }
                        cur_indx
                    } else {
                        break;
                    }
                }
            }
            Vertical => {
                // visit three to right of indx
                let mut cur_indx = indx;
                let mut cur_dist = dist;
                for step in 0..MAXSTEPS {
                    cur_indx = if let Some(cur_indx) = cur_indx.right() {
                        cur_dist += chart[cur_indx.row][cur_indx.col];
                        if step >= (MINSTEPS - 1)
                            && cur_dist < distance.horz[cur_indx.row][cur_indx.col]
                        {
                            pqueue.push(Node {
                                cost: cur_dist,
                                position: cur_indx,
                                direction: Horizontal,
                            });
                            distance.horz[cur_indx.row][cur_indx.col] = cur_dist;
                        }
                        cur_indx
                    } else {
                        break;
                    }
                }
                // visit three to left of indx
                let mut cur_indx = indx;
                let mut cur_dist = dist;
                for step in 0..MAXSTEPS {
                    cur_indx = if let Some(cur_indx) = cur_indx.left() {
                        cur_dist += chart[cur_indx.row][cur_indx.col];
                        if step >= (MINSTEPS - 1)
                            && cur_dist < distance.horz[cur_indx.row][cur_indx.col]
                        {
                            pqueue.push(Node {
                                cost: cur_dist,
                                position: cur_indx,
                                direction: Horizontal,
                            });
                            distance.horz[cur_indx.row][cur_indx.col] = cur_dist;
                        }
                        cur_indx
                    } else {
                        break;
                    }
                }
            }
        }
    }
    u16::MAX
}

fn day17_p1<const N: usize>(data: &str) -> u16 {
    find_path::<N, 1, 3>(data)
}

fn day17_p2<const N: usize>(data: &str) -> u16 {
    find_path::<N, 4, 10>(data)
}

pub fn run_day17_p1() -> u16 {
    let filename = "data/day_17.txt";
    let data = read_to_string(filename).unwrap();
    day17_p1::<141>(&data)
}

pub fn run_day17_p2() -> u16 {
    let filename = "data/day_17.txt";
    let data = read_to_string(filename).unwrap();
    day17_p2::<141>(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let now = Instant::now();
        let sol = run_day17_p1();
        let elapsed = now.elapsed().as_millis();
        println!("Day 17 part 1 solution is: {sol} in {elapsed} ms");
    } else {
        let now = Instant::now();
        let sol = run_day17_p2();
        let elapsed = now.elapsed().as_millis();
        println!("Day 17 part 2 solution is: {sol} in {elapsed} ms");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        2413432311323\n\
        3215453535623\n\
        3255245654254\n\
        3446585845452\n\
        4546657867536\n\
        1438598798454\n\
        4457876987766\n\
        3637877979653\n\
        4654967986887\n\
        4564679986453\n\
        1224686865563\n\
        2546548887735\n\
        4322674655533";

    #[test]
    fn test_day17_p1_example() {
        assert_eq!(day17_p1::<13>(EXAMPLE), 102);
    }

    #[test]
    fn test_day17_p2_example() {
        assert_eq!(day17_p2::<13>(EXAMPLE), 94)
    }

    #[test]
    fn test_day17_p1() {
        assert_eq!(run_day17_p1(), 1244);
    }

    #[test]
    fn test_day17_p2() {
        assert_eq!(run_day17_p2(), 1367);
    }
}
