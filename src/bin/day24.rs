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
    x: i128,
    y: i128,
    z: i128,
    quantity: PhantomData<Quantity>,
}

impl<Quantity> From<&str> for Vector<Quantity> {
    fn from(data: &str) -> Self {
        let [x, y, z]: [i128; 3] = data
            .split(',')
            .map(|t| t.trim().parse::<i128>().unwrap())
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
    fn intersection(&self, other: &Self) -> [Rational; 4] {
        fn intersection_x(
            px1: i128,
            py1: i128,
            px2: i128,
            py2: i128,
            vx1: i128,
            vy1: i128,
            vx2: i128,
            vy2: i128,
        ) -> Rational {
            let denom = vx2 * vy1 - vx1 * vy2;
            let numer = vx2 * vy1 * px1 - vx1 * vy2 * px2 + vx1 * vx2 * (py2 - py1);
            Rational { numer, denom }
        }
        let xi = intersection_x(
            self.pos.x,
            self.pos.y,
            other.pos.x,
            other.pos.y,
            self.vel.x,
            self.vel.y,
            other.vel.x,
            other.vel.y,
        );
        let yi = intersection_x(
            self.pos.y,
            self.pos.x,
            other.pos.y,
            other.pos.x,
            self.vel.y,
            self.vel.x,
            other.vel.y,
            other.vel.x,
        );
        let t1 = Rational {
            numer: xi.numer - xi.denom * self.pos.x,
            denom: xi.denom * self.vel.x,
        };
        let t2 = Rational {
            numer: xi.numer - xi.denom * other.pos.x,
            denom: xi.denom * other.vel.x,
        };
        [xi, yi, t1, t2]
    }
}

#[derive(Debug)]
struct Rational {
    numer: i128,
    denom: i128,
}

impl Rational {
    fn in_bounds(&self, lower: i128, upper: i128) -> bool {
        let Rational { numer, denom } = self;
        let numer = numer * denom.signum();
        let denom = denom * denom.signum();
        (numer >= lower * denom) && (numer <= upper * denom)
    }

    fn signum(&self) -> i128 {
        self.numer.signum() * self.denom.signum()
    }
}

fn day24_p1(data: &str, lower: i128, upper: i128) -> usize {
    let hailstones: Vec<Hailstone> = data.lines().map(Hailstone::from).collect();
    hailstones
        .iter()
        .enumerate()
        .map(|(ii, hailstone1)| {
            hailstones
                .iter()
                .skip(ii + 1)
                .filter(|hailstone2| {
                    let [ix, iy, t1, t2] = hailstone1.intersection(hailstone2);
                    if ix.denom == 0 {
                        if ix.numer == 0 {
                            panic!("Handle this case");
                        } else {
                            false
                        }
                    } else {
                        (t1.signum() > 0)
                            && (t2.signum() > 0)
                            && ix.in_bounds(lower, upper)
                            && iy.in_bounds(lower, upper)
                    }
                })
                .count()
        })
        .sum()
}

fn day24_p2(data: &str) -> i128 {
    let hailstones: Vec<Hailstone> = data.lines().map(Hailstone::from).collect();
}

pub fn run_day24_p1() -> usize {
    let filename = "data/day_24.txt";
    let data = read_to_string(filename).unwrap();
    day24_p1(&data, 200000000000000, 400000000000000)
}

pub fn run_day24_p2() -> i128 {
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
        assert_eq!(day24_p2(EXAMPLE), 47);
    }

    #[test]
    fn test_day24_p1() {
        assert_eq!(run_day24_p1(), 18098);
    }

    #[test]
    #[ignore]
    fn test_day24_p2() {
        // takes almost 2 minutes
        assert_eq!(run_day24_p2(), 0);
    }
}
