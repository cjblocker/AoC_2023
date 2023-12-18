//! Day 18: Lavaduct Lagoon

// I guess without the shoelace formula and Pick's theorem, this _may_ have been tricky
// we would have had to plot out the points (how big of an array?) and then march rays/flood fill or something.
// But its pretty easy this way!
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Point {
    fn from(tuple: (isize, isize)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

fn day18_p1(data: &str) -> usize {
    let mut last: Point = (0isize, 0isize).into();
    let mut area = 0;
    let mut boundary_points = 0;
    for line in data.lines() {
        // color is useless in part 1, but encodes the true data in part 2
        let (coord, _color) = line.rsplit_once(' ').unwrap();
        let (dir, steps) = coord.split_once(' ').unwrap();
        let steps = steps.parse::<isize>().unwrap();
        let next: Point = match dir {
            "L" => (last.x - steps, last.y),
            "R" => (last.x + steps, last.y),
            "U" => (last.x, last.y - steps),
            "D" => (last.x, last.y + steps),
            _ => unreachable!(),
        }
        .into();
        area += last.x * next.y - next.x * last.y; // Shoelace Formula
        last = next;
        boundary_points += steps;
    }
    // we connect back to origin, but (0,0) adds no area
    area = area.abs() / 2;
    let interior_points = area + 1 - boundary_points / 2; // Pick's Theorem
    (interior_points + boundary_points) as usize
}

fn day18_p2(data: &str) -> usize {
    // basically the same as part 1, just have to parse information differently
    let mut last: Point = (0isize, 0isize).into();
    let mut area = 0;
    let mut boundary_points = 0;
    for line in data.lines() {
        let color = line.split_once('#').unwrap().1.split_once(')').unwrap().0;
        let steps = isize::from_str_radix(&color[..5], 16).unwrap();
        let dir = &color[5..];
        let next: Point = match dir {
            "2" => (last.x - steps, last.y),
            "0" => (last.x + steps, last.y),
            "3" => (last.x, last.y - steps),
            "1" => (last.x, last.y + steps),
            _ => unreachable!(),
        }
        .into();
        area += last.x * next.y - next.x * last.y; // Shoelace Formula
        last = next;
        boundary_points += steps;
    }
    // we connect back to origin, but (0,0) adds no area
    area = area.abs() / 2;
    let interior_points = area + 1 - boundary_points / 2; // Pick's Theorem
    (interior_points + boundary_points) as usize
}

pub fn run_day18_p1() -> usize {
    let filename = "data/day_18.txt";
    let data = read_to_string(filename).unwrap();
    day18_p1(&data)
}

pub fn run_day18_p2() -> usize {
    let filename = "data/day_18.txt";
    let data = read_to_string(filename).unwrap();
    day18_p2(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let now = Instant::now();
        let sol = run_day18_p1();
        let elapsed = now.elapsed().as_millis();
        println!("Day 18 part 1 solution is: {sol} in {elapsed} ms");
    } else {
        let now = Instant::now();
        let sol = run_day18_p2();
        let elapsed = now.elapsed().as_millis();
        println!("Day 18 part 2 solution is: {sol} in {elapsed} ms");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        R 6 (#70c710)\n\
        D 5 (#0dc571)\n\
        L 2 (#5713f0)\n\
        D 2 (#d2c081)\n\
        R 2 (#59c680)\n\
        D 2 (#411b91)\n\
        L 5 (#8ceee2)\n\
        U 2 (#caa173)\n\
        L 1 (#1b58a2)\n\
        U 2 (#caa171)\n\
        R 2 (#7807d2)\n\
        U 3 (#a77fa3)\n\
        L 2 (#015232)\n\
        U 2 (#7a21e3)";

    #[test]
    fn test_day18_p1_example() {
        assert_eq!(day18_p1(EXAMPLE), 62);
    }

    #[test]
    fn test_day18_p2_example() {
        assert_eq!(day18_p2(EXAMPLE), 952408144115)
    }

    #[test]
    fn test_day18_p1() {
        assert_eq!(run_day18_p1(), 46394);
    }

    #[test]
    fn test_day18_p2() {
        assert_eq!(run_day18_p2(), 201398068194715);
    }
}
