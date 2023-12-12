//! Day 9: Mirage Maintenance
use std::env;
use std::fs::read_to_string;

fn day11_p1(data: &str) -> i64 {
    // Note that part 2 completely supersedes part 1. I'm leaving this solution
    // here for historical reasons. It's how I approached the problem originally.
    let mut starmap: Vec<Vec<char>> = data
        .lines()
        .flat_map(|line| {
            let line: Vec<char> = line.chars().collect();
            if line.iter().all(|x| *x == '.') {
                vec![line.clone(), line]
            } else {
                vec![line]
            }
        })
        .collect();
    let width = starmap.first().unwrap().len();
    for jj in (0..width).rev() {
        if starmap.iter().all(|line| line[jj] == '.') {
            for line in starmap.iter_mut() {
                line.insert(jj, '.');
            }
        }
    }
    let mut galaxies = vec![];
    for (ii, line) in starmap.iter().enumerate() {
        for (jj, c) in line.iter().enumerate() {
            if *c == '#' {
                galaxies.push((ii as i64, jj as i64));
            }
        }
    }
    let mut dist = 0;
    for g1 in galaxies.iter() {
        for g2 in galaxies.iter() {
            dist += (g1.0 - g2.0).abs() + (g1.1 - g2.1).abs()
        }
    }
    // we counted each pair twice for simplicity
    dist / 2
}

/// Turn a vector of widths into indices/offsets by accumulating
fn accumulate(mut x: Vec<i64>) -> Vec<i64> {
    let mut running_sum = 0;
    for ii in x.iter_mut() {
        let tmp = *ii;
        *ii = running_sum;
        running_sum += tmp;
    }
    x
}

fn day11_p2(data: &str, expand: i64) -> i64 {
    // Idea: We could use the separability of the L1 metric to reduce this 2D problems
    // into two 1D problems. I didn't do that here, but might be simpler, probably not faster.
    let starmap: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let width = starmap.first().unwrap().len();
    let mut xidx = vec![1; width];
    let mut yidx = vec![1; starmap.len()];
    for (ii, line) in starmap.iter().enumerate() {
        if line.iter().all(|x| *x == '.') {
            yidx[ii] = expand;
        }
    }
    for jj in (0..width).rev() {
        if starmap.iter().all(|line| line[jj] == '.') {
            xidx[jj] = expand;
        }
    }
    xidx = accumulate(xidx);
    yidx = accumulate(yidx);

    let mut galaxies = vec![];
    for (ii, line) in yidx.iter().zip(starmap.iter()) {
        for (jj, c) in xidx.iter().zip(line.iter()) {
            if *c == '#' {
                galaxies.push((*ii, *jj));
            }
        }
    }
    let mut dist = 0;
    for g1 in galaxies.iter() {
        for g2 in galaxies.iter() {
            dist += (g1.0 - g2.0).abs() + (g1.1 - g2.1).abs()
        }
    }
    // we counted each pair twice for simplicity
    dist / 2
}

pub fn run_day11_p1() -> i64 {
    let filename = "data/day_11.txt";
    let data = read_to_string(filename).unwrap();
    day11_p1(&data)
}

pub fn run_day11_p2() -> i64 {
    let filename = "data/day_11.txt";
    let data = read_to_string(filename).unwrap();
    day11_p2(&data, 1_000_000)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let sol = run_day11_p1();
        println!("Day 11 part 1 solution is: {sol}");
    } else {
        let sol = run_day11_p2();
        println!("Day 11 part 2 solution is: {sol}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....";

    #[test]
    fn test_day11_p1_example() {
        assert_eq!(day11_p1(EXAMPLE), 374)
    }

    #[test]
    fn test_day11_p2_example1() {
        assert_eq!(day11_p2(EXAMPLE, 2), 374)
    }
    #[test]
    fn test_day11_p2_example2() {
        assert_eq!(day11_p2(EXAMPLE, 10), 1030)
    }
    #[test]
    fn test_day11_p2_example3() {
        assert_eq!(day11_p2(EXAMPLE, 100), 8410)
    }

    #[test]
    fn test_day11_p1() {
        assert_eq!(run_day11_p1(), 9233514)
    }

    #[test]
    fn test_day11_p2() {
        assert_eq!(run_day11_p2(), 363293506944)
    }
}
