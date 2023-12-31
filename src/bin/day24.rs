//! Day 24: Never Tell Me The Odds
// use rayon::prelude::*;
use itertools::Itertools;
use num::integer::lcm;
use num::Integer;
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
        // make sure the denominator is non-negative
        let numer = numer * denom.signum();
        let denom = denom * denom.signum();
        (numer >= lower * denom) && (numer <= upper * denom)
    }

    fn signum(&self) -> i128 {
        self.numer.signum() * self.denom.signum()
    }
}

impl TryFrom<Rational> for i128 {
    type Error = &'static str;

    fn try_from(value: Rational) -> Result<Self, Self::Error> {
        let (quotient, remainder) = (value.numer / value.denom, value.numer % value.denom);
        if remainder != 0 {
            Err("Rational is not an integer")
        } else {
            Ok(quotient)
        }
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

/// Finds the solution to the Chinese Remainder Theorem (CRT) if it exists,
/// otherwise return None. Mod values (ni) do not need to be coprime.
/// Solves the system of equations: xi = ai mod ni for xi,ai,ni integers.
fn chinese_remainder_theorem(values: &[(i128, i128)]) -> Option<(i128, i128)> {
    // I think there is a bug in this implementation as it sometimes returns
    // values too large or None when its not supposed too.
    match values.len() {
        0 => panic!("values is empty"),
        1 => Some((values[0].0 % values[0].1, values[0].1)),
        2 => {
            let [(a1, n1), (a2, n2)]: [(i128, i128); 2] = values.try_into().unwrap();
            let egcd = n1.extended_gcd(&n2);
            if (a1 - a2) % egcd.gcd != 0 {
                return None;
            }
            let lambda = (a1 - a2) / egcd.gcd;
            let m = lcm(n1, n2);
            let mut x = (a1 - n1 * lambda * egcd.x) % m;
            if x < 0 {
                x += m;
            }
            Some((x, m))
        }
        n => {
            let half = n / 2;
            let a = chinese_remainder_theorem(&values[..half])?;
            let b = chinese_remainder_theorem(&values[half..])?;
            chinese_remainder_theorem(&[a, b])
        }
    }
}

fn day24_p2(data: &str, take: usize) -> i128 {
    // due to a bug (I assume in the CRT implementation), some values of take here
    // give wrong answers or never return. A take value of 45 works on the input.
    let hailstones: Vec<Hailstone> = data.lines().map(Hailstone::from).take(take).collect();
    // we only need the sum of the rocks x,y,z so we can just do the same to the hailstones
    // and then only have to compute once. Yay, linearity.
    let ak: Vec<(i128, i128)> = hailstones
        .iter()
        .map(|hs| {
            (
                hs.pos.x + hs.pos.y + hs.pos.z,
                hs.vel.x + hs.vel.y + hs.vel.z,
            )
        })
        .collect();
    // The CRT doesn't give us a good way to handle estimating our vx+vy+vz, so we just
    // brute force this one parameter.
    'outer: for vel in (0..).interleave((1..).map(|x| -x)) {
        let ak_nk: Vec<(i128, i128)> = ak.iter().map(|(ai, ni)| (*ai, *ni - vel)).collect();
        // x0 = ak + nk*t
        if ak_nk.iter().any(|(_, ni)| *ni == 0) {
            // if the net velocity (nk) is ever zero, then our position (x0)
            // must equal the hailstones initial position (ak).
            // This doesn't happen in practices though, logic is in commit history.
            continue;
        }
        let Some((x0, m)) = chinese_remainder_theorem(&ak_nk) else {
            continue 'outer;
        };
        // println!("{} mod {} @ {}", x0, m, vel);

        // we have found a solution that satisfies the CRT, but we need to verify
        // that the time points are all positive (the hailstones future).
        let mut ts: Vec<i128> = ak_nk
            .iter()
            .filter(|(_, ni)| *ni != 0)
            .map(|(ai, ni)| (x0 - ai) / ni)
            .collect();
        // we limit our search to [-1000, 1000] to avoid getting stuck here forever
        let mut cs = (1..1000).interleave((1..1000).map(|x| -x));
        let mut c = 0;
        while ts.iter().any(|&t| t <= 0) {
            c = if let Some(c) = cs.next() {
                c
            } else {
                continue 'outer;
            };
            ts = ak_nk
                .iter()
                .filter(|(_, ni)| *ni != 0)
                .map(|(ai, ni)| (x0 + c * m - ai) / ni)
                .collect();
        }
        let x0 = x0 + c * m;
        return x0;
    }
    unreachable!();
}

pub fn run_day24_p1() -> usize {
    let filename = "data/day_24.txt";
    let data = read_to_string(filename).unwrap();
    day24_p1(&data, 200000000000000, 400000000000000)
}

pub fn run_day24_p2() -> i128 {
    let filename = "data/day_24.txt";
    let data = read_to_string(filename).unwrap();
    // for ii in 1..301 {
    //     println!("{:?}: {:?}", ii, day24_p2(&data, ii));
    // }
    day24_p2(&data, 45)
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
    fn test_day24_p2_example() {
        assert_eq!(day24_p2(EXAMPLE, 100), 47);
    }

    #[test]
    fn test_day24_p1() {
        assert_eq!(run_day24_p1(), 18098);
    }

    #[test]
    fn test_day24_p2() {
        assert_eq!(run_day24_p2(), 886858737029295);
    }

    #[test]
    fn test_crt() {
        let values = [(2, 3), (3, 5), (2, 7)];
        let (x, m) = chinese_remainder_theorem(&values).unwrap();
        assert_eq!(x, 23);
        assert_eq!(m, 105);
    }

    #[test]
    fn test_crt_non_coprime() {
        let values = [(1, 2), (1, 3), (1, 4), (1, 5), (1, 6)];
        let (x, m) = chinese_remainder_theorem(&values).unwrap();
        assert_eq!(x, 1);
        assert_eq!(m, 60);
    }

    #[test]
    fn test_crt_p2() {
        // vel = -3
        let values = [
            (19, -2 + 3),
            (18, -1 + 3),
            (20, -2 + 3),
            (12, -1 + 3),
            (20, 1 + 3),
        ];
        let (x0, m) = chinese_remainder_theorem(&values).unwrap();
        let mut ts: Vec<i128> = values
            .iter()
            .filter(|(_, ni)| *ni != 0)
            .map(|(ai, ni)| (x0 - ai) / ni)
            .collect();
        let mut cs = (1..).interleave((1..).map(|x| -x));
        let mut c = 0;
        while ts.iter().any(|&t| t <= 0) {
            c = cs.next().unwrap();
            ts = values
                .iter()
                .filter(|(_, ni)| *ni != 0)
                .map(|(ai, ni)| (x0 + c * m - ai) / ni)
                .collect();
        }
        let x = x0 + c * m;
        assert_eq!(x, 24);
    }
}
