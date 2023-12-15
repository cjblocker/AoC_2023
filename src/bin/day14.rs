//! Day 14: Parabolic Reflector Dish
#![allow(clippy::needless_range_loop)]
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

fn day14_p1(data: &str) -> usize {
    let chart: Vec<&[u8]> = data.lines().map(|line| line.as_bytes()).collect();
    let height = chart.len();
    let width = chart[0].len();

    let mut load = 0;
    for jj in 0..width {
        let mut backstop = 0;
        for ii in 0..height {
            match chart[ii][jj] {
                b'O' => {
                    load += height - backstop;
                    backstop += 1;
                }
                b'#' => backstop = ii + 1,
                b'.' => (),
                _c => panic!("Unrecognized char {:?}", _c),
            }
        }
    }
    load
}

fn north_tilt(chart: &mut Vec<Vec<u8>>) {
    let height = chart.len();
    let width = chart[0].len();
    // North Tilt
    for jj in 0..width {
        let mut backstop = 0;
        for ii in 0..height {
            match chart[ii][jj] {
                b'O' => {
                    chart[backstop][jj] = b'O';
                    backstop += 1;
                }
                b'#' => {
                    for kk in backstop..ii {
                        chart[kk][jj] = b'.';
                    }
                    backstop = ii + 1;
                }
                b'.' => (),
                _c => panic!("Unrecognized char {:?}", _c),
            }
        }
        for kk in backstop..height {
            chart[kk][jj] = b'.';
        }
    }
}

fn west_tilt(chart: &mut Vec<Vec<u8>>) {
    let height = chart.len();
    let width = chart[0].len();
    // West Tilt
    for ii in 0..height {
        let mut backstop = 0;
        for jj in 0..width {
            match chart[ii][jj] {
                b'O' => {
                    chart[ii][backstop] = b'O';
                    backstop += 1;
                }
                b'#' => {
                    for kk in backstop..jj {
                        chart[ii][kk] = b'.';
                    }
                    backstop = jj + 1;
                }
                b'.' => (),
                _c => panic!("Unrecognized char {:?}", _c),
            }
        }
        for kk in backstop..height {
            chart[ii][kk] = b'.';
        }
    }
}

fn south_tilt(chart: &mut Vec<Vec<u8>>) {
    let height = chart.len();
    let width = chart[0].len();
    // South Tilt
    for jj in 0..width {
        let mut backstop = height;
        for ii in (0..height).rev() {
            match chart[ii][jj] {
                b'O' => {
                    chart[backstop - 1][jj] = b'O';
                    backstop -= 1;
                }
                b'#' => {
                    for kk in (ii + 1)..backstop {
                        chart[kk][jj] = b'.';
                    }
                    backstop = ii;
                }
                b'.' => (),
                _c => panic!("Unrecognized char {:?}", _c),
            }
        }
        for kk in 0..backstop {
            chart[kk][jj] = b'.';
        }
    }
}

fn east_tilt(chart: &mut Vec<Vec<u8>>) {
    let height = chart.len();
    let width = chart[0].len();
    // East Tilt
    for ii in 0..height {
        let mut backstop = width;
        for jj in (0..width).rev() {
            match chart[ii][jj] {
                b'O' => {
                    chart[ii][backstop - 1] = b'O';
                    backstop -= 1;
                }
                b'#' => {
                    for kk in (jj + 1)..backstop {
                        chart[ii][kk] = b'.';
                    }
                    backstop = jj;
                }
                b'.' => (),
                _c => panic!("Unrecognized char {:?}", _c),
            }
        }
        for kk in 0..backstop {
            chart[ii][kk] = b'.';
        }
    }
}

fn cycle(chart: &mut Vec<Vec<u8>>) {
    north_tilt(chart);
    west_tilt(chart);
    south_tilt(chart);
    east_tilt(chart);
}

fn compute_load(chart: &Vec<Vec<u8>>) -> usize {
    let height = chart.len();
    let width = chart[0].len();

    let mut load = 0;
    for ii in 0..height {
        for jj in 0..width {
            match chart[ii][jj] {
                b'O' => {
                    load += height - ii;
                }
                b'#' => (),
                b'.' => (),
                _c => panic!("Unrecognized char {:?}", _c),
            }
        }
    }
    load
}

#[allow(dead_code)]
fn print_chart(chart: &[Vec<u8>]) {
    for line in chart.iter() {
        println!("{}", std::str::from_utf8(line).unwrap());
    }
    println!();
}

fn day14_p2(data: &str) -> usize {
    const WARMUP: usize = 151;
    let mut chart = data
        .lines()
        .map(|line| line.to_owned().into_bytes())
        .collect::<Vec<_>>();

    // get to the point where it is periodic
    for _ in 0..WARMUP {
        cycle(&mut chart);
    }

    // detect loading cycle (I'm detecting a cycle in the load which
    // is generally only necessary, not sufficient)
    let mut load_cycle = vec![compute_load(&chart)];
    let mut loop_ptr = 0;
    let mut end = 2;
    loop {
        cycle(&mut chart);
        let load = compute_load(&chart);
        if load_cycle[loop_ptr] == load {
            if loop_ptr == 0 {
                end = load_cycle.len();
            } else if loop_ptr == end {
                break;
            }
            loop_ptr += 1;
        } else {
            loop_ptr = 0;
            if load_cycle[0] == load {
                end = load_cycle.len();
                loop_ptr += 1;
            }
        }
        load_cycle.push(load);
    }
    load_cycle[(1_000_000_000 - WARMUP) % load_cycle.len()]
}

pub fn run_day14_p1() -> usize {
    let filename = "data/day_14.txt";
    let data = read_to_string(filename).unwrap();
    day14_p1(&data)
}

pub fn run_day14_p2() -> usize {
    let filename = "data/day_14.txt";
    let data = read_to_string(filename).unwrap();
    day14_p2(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let now = Instant::now();
        let sol = run_day14_p1();
        let elapsed = now.elapsed().as_secs();
        println!("Day 14 part 1 solution is: {sol} in {elapsed} seconds");
    } else {
        let now = Instant::now();
        let sol = run_day14_p2();
        let elapsed = now.elapsed().as_secs();
        println!("Day 14 part 2 solution is: {sol} in {elapsed} seconds");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        O....#....\n\
        O.OO#....#\n\
        .....##...\n\
        OO.#O....O\n\
        .O.....O#.\n\
        O.#..O.#.#\n\
        ..O..#O..O\n\
        .......O..\n\
        #....###..\n\
        #OO..#....";

    #[test]
    fn test_day14_p1_example() {
        assert_eq!(day14_p1(EXAMPLE), 136);
    }

    #[test]
    fn test_day14_p2_example() {
        assert_eq!(day14_p2(EXAMPLE), 64)
    }

    #[test]
    fn test_day14_p1() {
        assert_eq!(run_day14_p1(), 110677);
    }

    #[test]
    fn test_day14_p2() {
        assert_eq!(run_day14_p2(), 90551);
    }

    #[test]
    fn test_day14_p2_solves_p1() {
        let mut chart = EXAMPLE
            .lines()
            .map(|line| line.to_owned().into_bytes())
            .collect::<Vec<_>>();

        north_tilt(&mut chart);
        let sol = compute_load(&chart);
        assert_eq!(sol, 136);
    }
}
