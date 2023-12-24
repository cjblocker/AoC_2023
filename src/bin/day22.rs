//! Day 22: Sand Slabs
// #![allow(dead_code)]
use core::ops::Range;
use std::cmp::Ordering;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Brick {
    x: Range<u16>,
    y: Range<u16>,
    z: Range<u16>,
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.z
            .start
            .cmp(&other.z.start)
            .then_with(|| self.z.end.cmp(&other.z.end))
            .then_with(|| self.x.start.cmp(&other.x.start))
            .then_with(|| self.x.end.cmp(&other.x.end))
            .then_with(|| self.y.start.cmp(&other.y.start))
            .then_with(|| self.y.end.cmp(&other.y.end))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Brick {
    fn from(data: &str) -> Self {
        let (start, end) = data.split_once('~').unwrap();
        let start: Vec<u16> = start
            .split(',')
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
        let end: Vec<u16> = end
            .split(',')
            .map(|x| x.parse::<u16>().unwrap() + 1)
            .collect();
        Self {
            x: start[0]..end[0],
            y: start[1]..end[1],
            z: start[2]..end[2],
        }
    }
}

impl Brick {
    fn move_down(&mut self, steps: u16) {
        self.z.start -= steps;
        self.z.end -= steps;
    }

    fn xy_overlap(&self, other: &Self) -> bool {
        (self.x.end > other.x.start && other.x.end > self.x.start)
            && (self.y.end > other.y.start && other.y.end > self.y.start)
    }

    fn supports(&self, other: &Self) -> bool {
        self.z.end == other.z.start && self.xy_overlap(other)
    }
}

fn settle_bricks(bricks: &mut Vec<Brick>) {
    bricks.sort_unstable();
    // settle the bricks
    let mut changed = true;
    while changed {
        changed = false;
        let mut prev_layer: Vec<&Brick> = vec![];
        let mut this_layer: Vec<&Brick> = vec![];
        let mut z_level = 1;
        for brick in bricks.iter_mut() {
            if brick.z.start != z_level {
                z_level = brick.z.start;
                prev_layer = prev_layer
                    .into_iter()
                    .chain(this_layer)
                    .filter(|x| x.z.contains(&(z_level - 1)))
                    .collect();
                this_layer = vec![];
            }
            if z_level > 1 {
                if prev_layer.is_empty()
                    || !prev_layer
                        .iter()
                        .filter(|b| b.z.end == z_level)
                        .any(|b| b.xy_overlap(brick))
                {
                    brick.move_down(1);
                    changed = true;
                    prev_layer.push(brick);
                } else {
                    this_layer.push(brick);
                }
            } else {
                this_layer.push(brick);
            }

            assert!(prev_layer.iter().all(|b| b.z.contains(&(z_level - 1))));
            assert!(this_layer.iter().all(|b| b.z.contains(&z_level)));
            assert!(prev_layer.iter().all(|b| prev_layer
                .iter()
                .filter(|b2| b.xy_overlap(b2))
                .count()
                == 1));
            assert!(this_layer.iter().all(|b| this_layer
                .iter()
                .filter(|b2| b.xy_overlap(b2))
                .count()
                == 1));
        }
        // the need to re-sort is subtle
        bricks.sort_unstable();
    }
}

fn day22_p1(data: &str) -> u64 {
    let mut bricks: Vec<Brick> = data.lines().map(Brick::from).collect();
    settle_bricks(&mut bricks);
    // dbg!(&bricks);

    bricks
        .iter()
        .enumerate()
        .map(|(ii, brick)| {
            let count = bricks
                .iter()
                .skip(ii)
                .filter(|b| brick.supports(b)) // of all bricks supported by this brick
                .map(|supported| {
                    // is this supported brick only supported by 1 brick (ie this brick)
                    (bricks.iter().filter(|b| b.supports(supported)).count() == 1) as usize
                })
                .sum::<usize>(); // return how many brick are uniquely supported by this brick
                                 // dbg!(count)
            count
        })
        .filter(|&x| x == 0) // how many brick are not uniquely supporting a brick?
        .count() as u64
}

fn day22_p2(data: &str) -> u64 {
    let mut bricks: Vec<Brick> = data.lines().map(Brick::from).collect();
    settle_bricks(&mut bricks);
    0
}

pub fn run_day22_p1() -> u64 {
    let filename = "data/day_22.txt";
    let data = read_to_string(filename).unwrap();
    day22_p1(&data)
}

pub fn run_day22_p2() -> u64 {
    let filename = "data/day_22.txt";
    let data = read_to_string(filename).unwrap();
    day22_p2(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let now = Instant::now();
        let sol = run_day22_p1();
        let elapsed = now.elapsed().as_millis();
        println!("Day 22 part 1 solution is: {sol} in {elapsed} ms");
    } else {
        let now = Instant::now();
        let sol = run_day22_p2();
        let elapsed = now.elapsed().as_millis();
        println!("Day 22 part 2 solution is: {sol} in {elapsed} ms");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        1,0,1~1,2,1\n\
        0,0,2~2,0,2\n\
        0,2,3~2,2,3\n\
        0,0,4~0,2,4\n\
        2,0,5~2,2,5\n\
        0,1,6~2,1,6\n\
        1,1,8~1,1,9";

    #[test]
    fn test_day22_p1_example() {
        assert_eq!(day22_p1(EXAMPLE), 5);
    }

    #[test]
    fn test_day22_p2_example() {
        assert_eq!(day22_p2(EXAMPLE), 7);
    }

    #[test]
    fn test_day22_p1() {
        assert_eq!(run_day22_p1(), 485);
    }

    #[test]
    #[ignore]
    fn test_day22_p2() {
        assert_eq!(run_day22_p2(), 0);
    }
}
