use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

/// Find all numbers in the chart that are adjacent to a symbol including diagonally
fn day10_p1(chart: &str) -> u32 {
    let chart: Vec<Vec<char>> = chart
        .lines()
        .map(|line| {
            let mut line: Vec<char> = line.chars().collect();
            line.push('.'); // avoid line-end boundary condition
            line
        })
        .collect();

    let mut start = None;
    for (ii, line) in chart.iter().enumerate() {
        for (jj, c) in line.iter().enumerate() {
            if *c == 'S' {
                start = Some((ii, jj));
                break;
            }
        }
    }
    let start = start.unwrap();

    let mut pos = (start.0, start.1 + 1);
    let mut dir = Direction::East;
    let mut pipe = 'J';
    let mut count = 1;
    while pipe != 'S' {
        dir = match (pipe, dir) {
            ('-', Direction::East) => Direction::East,
            ('-', Direction::West) => Direction::West,
            ('|', Direction::North) => Direction::North,
            ('|', Direction::South) => Direction::South,
            ('L', Direction::South) => Direction::East,
            ('L', Direction::West) => Direction::North,
            ('J', Direction::South) => Direction::West,
            ('J', Direction::East) => Direction::North,
            ('7', Direction::North) => Direction::West,
            ('7', Direction::East) => Direction::South,
            ('F', Direction::North) => Direction::East,
            ('F', Direction::West) => Direction::South,
            _ => panic!("Error can't follow pipe: {:?},{:?}", pipe, dir),
        };
        match dir {
            Direction::North => pos.0 -= 1,
            Direction::South => pos.0 += 1,
            Direction::East => pos.1 += 1,
            Direction::West => pos.1 -= 1,
        };
        pipe = chart[pos.0][pos.1];
        count += 1;
    }
    dbg!(count);
    count / 2
}

/// A gear is a '*' with exactly two numbers by it
/// find all gears, multiply their two numbers, and add them up
fn day10_p2(chart: &str) -> u32 {
    let chart: Vec<Vec<char>> = chart
        .lines()
        .map(|line| {
            let mut line: Vec<char> = line.chars().collect();
            line.push('.'); // avoid line-end boundary condition
            line
        })
        .collect();

    let mut start = None;
    for (ii, line) in chart.iter().enumerate() {
        for (jj, c) in line.iter().enumerate() {
            if *c == 'S' {
                start = Some((ii, jj));
                break;
            }
        }
    }
    let start = start.unwrap();

    let mut pos = (start.0, start.1 + 1);
    let mut dir = Direction::East;
    let mut pipe = 'J';
    let mut rot: i32 = 1;
    let mut path = vec![(pos.clone(), pipe, dir)];
    while pipe != 'S' {
        dir = match (pipe, dir) {
            ('-', Direction::East) => Direction::East,
            ('-', Direction::West) => Direction::West,
            ('|', Direction::North) => Direction::North,
            ('|', Direction::South) => Direction::South,
            ('L', Direction::South) => {
                rot += 1;
                Direction::East
            }
            ('L', Direction::West) => {
                rot -= 1;
                Direction::North
            }
            ('J', Direction::South) => {
                rot -= 1;
                Direction::West
            }
            ('J', Direction::East) => {
                rot += 1;
                Direction::North
            }
            ('7', Direction::North) => {
                rot += 1;
                Direction::West
            }
            ('7', Direction::East) => {
                rot -= 1;
                Direction::South
            }
            ('F', Direction::North) => {
                rot -= 1;
                Direction::East
            }
            ('F', Direction::West) => {
                rot += 1;
                Direction::South
            }
            _ => panic!("Error can't follow pipe: {:?},{:?}", pipe, dir),
        };
        match dir {
            Direction::North => pos.0 -= 1,
            Direction::South => pos.0 += 1,
            Direction::East => pos.1 += 1,
            Direction::West => pos.1 -= 1,
        };
        pipe = chart[pos.0][pos.1];
        path.push((pos.clone(), pipe, dir));
    }
    let path = path; // no longer mutable
    let path_points: Vec<(usize, usize)> = path.iter().map(|x| x.0).collect();
    let ccw = rot.signum() > 0; // +1 for CCW, -1 for clockwise
    let mut inside = vec![];
    for (pos, pipe, dir) in path.iter() {
        let mut potential_inside = vec![];

        match (ccw, pipe, dir) {
            (true, '-', Direction::East) | (false, '-', Direction::West) => {
                potential_inside.push((pos.0 - 1, pos.1));
            }
            (true, '-', Direction::West) | (false, '-', Direction::East) => {
                potential_inside.push((pos.0 + 1, pos.1));
            }
            (true, '|', Direction::North) | (false, '|', Direction::South) => {
                potential_inside.push((pos.0, pos.1 - 1));
            }
            (true, '|', Direction::South) | (false, '|', Direction::North) => {
                potential_inside.push((pos.0, pos.1 + 1));
            }
            (true, 'L', Direction::South) | (false, 'L', Direction::West) => {
                potential_inside.push((pos.0 - 1, pos.1 + 1));
            }
            (true, 'L', Direction::West) | (false, 'L', Direction::South) => {
                potential_inside.push((pos.0, pos.1 - 1));
                potential_inside.push((pos.0 + 1, pos.1 - 1));
                potential_inside.push((pos.0 + 1, pos.1));
            }
            (true, 'J', Direction::East) | (false, 'J', Direction::South) => {
                potential_inside.push((pos.0 - 1, pos.1 - 1));
            }
            (true, 'J', Direction::South) | (false, 'J', Direction::East) => {
                potential_inside.push((pos.0, pos.1 + 1));
                potential_inside.push((pos.0 + 1, pos.1 + 1));
                potential_inside.push((pos.0 + 1, pos.1));
            }
            (true, '7', Direction::North) | (false, '7', Direction::East) => {
                potential_inside.push((pos.0 + 1, pos.1 - 1));
            }
            (true, '7', Direction::East) | (false, '7', Direction::North) => {
                potential_inside.push((pos.0, pos.1 + 1));
                potential_inside.push((pos.0 - 1, pos.1 + 1));
                potential_inside.push((pos.0 - 1, pos.1));
            }
            (true, 'F', Direction::West) | (false, 'F', Direction::North) => {
                potential_inside.push((pos.0 + 1, pos.1 + 1));
            }
            (true, 'F', Direction::North) | (false, 'F', Direction::West) => {
                potential_inside.push((pos.0, pos.1 - 1));
                potential_inside.push((pos.0 - 1, pos.1 - 1));
                potential_inside.push((pos.0 - 1, pos.1));
            }
            (_, 'S', _) => {}
            _ => panic!("Error can't follow pipe: {:?},{:?}", pipe, dir),
        };

        while let Some(p) = potential_inside.pop() {
            if path_points.contains(&p) || inside.contains(&p) {
                continue;
            }
            // must be a valid point
            inside.push(p);
            // add all of its neighbors
            potential_inside.push((p.0 - 1, p.1 - 1));
            potential_inside.push((p.0 - 1, p.1));
            potential_inside.push((p.0 - 1, p.1 + 1));
            potential_inside.push((p.0, p.1 - 1));
            // potential_inside.push((p.0, p.1));
            potential_inside.push((p.0, p.1 + 1));
            potential_inside.push((p.0 + 1, p.1 - 1));
            potential_inside.push((p.0 + 1, p.1));
            potential_inside.push((p.0 + 1, p.1 + 1));
        }
    }
    inside.len() as u32
}

pub fn run_day10_p1() -> u32 {
    let filename = "data/day_10.txt";
    let chart = read_to_string(filename).unwrap();
    day10_p1(&chart)
}

pub fn run_day10_p2() -> u32 {
    let filename = "data/day_10.txt";
    let chart = read_to_string(filename).unwrap();
    day10_p2(&chart)
}

use std::env;
fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let sol = run_day10_p1();
        println!("Day 10 part 1 solution is: {sol}");
    } else {
        let sol = run_day10_p2();
        println!("Day 10 part 2 solution is: {sol}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        7-F7-\n\
        .FJ|7\n\
        SJLL7\n\
        |F--J\n\
        LJ.LJ";

    const EXAMPLE2: &str = "\
        ...........\n\
        .F-------7.\n\
        .|F-----7|.\n\
        .||.....||.\n\
        .||.....||.\n\
        .|L-7.F-J|.\n\
        .|..|.|..|.\n\
        .L-SJ.L--J.\n\
        ...........";

    const EXAMPLE3: &str = "\
        .F----7F7F7F7F-7....\n\
        .|F--7||||||||FJ....\n\
        .||.FJ||||||||L7....\n\
        FJL7L7LJLJ||LJ.L-7..\n\
        L--J.L7...LJF7F-7L7.\n\
        ....F-J..F7SJ|L7L7L7\n\
        ....L7.F7||L7|.L7L7|\n\
        .....|FJLJ|FJ|F7|.LJ\n\
        ....FJL-7.||.||||...\n\
        ....L---J.LJ.LJLJ...";

    const EXAMPLE4: &str = "\
        FF7F7F7F7F7F7F7F---7\n\
        L|SJ||||||||||||F--J\n\
        FL-7LJLJ||||||LJL-77\n\
        F--JF--7||LJLJ7F7FJ-\n\
        L---JF-JLJ.||-FJLJJ7\n\
        |F|F-JF---7F7-L7L|7|\n\
        |FFJF7L7F-JF7|JL---7\n\
        7-L-JL7||F7|L7F-7F7|\n\
        L.L7LFJ|||||FJL7||LJ\n\
        L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_day10_p1_example() {
        assert_eq!(day10_p1(EXAMPLE), 8)
    }

    #[test]
    fn test_day10_p2_example() {
        assert_eq!(day10_p2(EXAMPLE), 1)
    }

    #[test]
    fn test_day10_p2_example2() {
        assert_eq!(day10_p2(EXAMPLE2), 4)
    }

    #[test]
    fn test_day10_p2_example3() {
        assert_eq!(day10_p2(EXAMPLE3), 8)
    }

    #[test]
    fn test_day10_p2_example4() {
        assert_eq!(day10_p2(EXAMPLE4), 10)
    }

    #[test]
    fn test_day10_p1() {
        assert_eq!(run_day10_p1(), 6823)
    }

    #[test]
    fn test_day10_p2() {
        assert_eq!(run_day10_p2(), 415)
    }
}
