//! Day 8: Haunted Wasteland
use num::integer::lcm;
use std::collections::HashMap;
use std::convert::From;
use std::env;
use std::fs::read_to_string;

struct Chart {
    turns: String,
    connections: HashMap<String, Turn>,
}

struct Turn {
    left: String,
    right: String,
}

impl From<&str> for Chart {
    fn from(item: &str) -> Self {
        let (turns, connections) = item.split_once("\n\n").unwrap();
        let turns = turns.trim().to_owned();
        let connections: HashMap<String, Turn> = connections
            .lines()
            .map(|line| {
                let (key, values) = line.split_once(" = ").unwrap();
                let key = key.trim().to_owned();
                let (lval, rval) = values.split_once(", ").unwrap();
                (
                    key,
                    Turn {
                        left: lval.replace('(', "").trim().to_owned(),
                        right: rval.replace(')', "").trim().to_owned(),
                    },
                )
            })
            .collect();
        Self { turns, connections }
    }
}

/// Starting from Node AAA traverse the graph to Node ZZZ following
/// the given turn directions and report number of steps
fn day08_p1(chart: &str) -> u64 {
    // I think I need to destructure this struct so I can borrow independently?
    let Chart { turns, connections } = Chart::from(chart);
    let mut key = "AAA".to_string();
    let mut steps = 0;
    for (step, direction) in turns.chars().cycle().enumerate() {
        key = match direction {
            'L' => connections.get(&key).unwrap().left.to_owned(),
            'R' => connections.get(&key).unwrap().right.to_owned(),
            _ => panic!("Unknown direction"),
        };
        if key == "ZZZ" {
            steps = step + 1;
            break;
        }
    }
    steps as u64
}

/// Starting from all nodes that end in 'A', traverse the graph following the given
/// turn directions until all of the nodes end in 'Z' and report the number of steps
fn day08_p2(chart: &str) -> u64 {
    // For this problem, we make very specific assumptions
    // based off the structure of the *given* input (outside of the problem description).
    // specifically each starting point --A puts you in a loop that contains a single --Z
    // exit node. The cycle period happens to be the distance from --A to --Z. We find
    // each cycle length and then compute their least common multiple (LCM).
    let Chart { turns, connections } = Chart::from(chart);
    let mut keys: Vec<String> = connections
        .keys()
        .filter(|key| key.as_bytes()[2] == b'A')
        .map(|key| key.to_owned())
        .collect();
    let mut cycle_lens = vec![];
    for (step, direction) in turns.chars().cycle().enumerate() {
        keys = keys
            .into_iter()
            .map(|key| match direction {
                'L' => connections.get(&key).unwrap().left.to_owned(),
                'R' => connections.get(&key).unwrap().right.to_owned(),
                _ => panic!("Unknown direction"),
            })
            .filter_map(|key| {
                if key.as_bytes()[2] == b'Z' {
                    cycle_lens.push(step as u64 + 1);
                    None
                } else {
                    Some(key)
                }
            })
            .collect();
        if keys.is_empty() {
            break;
        }
    }
    cycle_lens.into_iter().fold(1, lcm)
}

pub fn run_day08_p1() -> u64 {
    let filename = "data/day_08.txt";
    let chart = read_to_string(filename).unwrap();
    day08_p1(&chart)
}

pub fn run_day08_p2() -> u64 {
    let filename = "data/day_08.txt";
    let chart = read_to_string(filename).unwrap();
    day08_p2(&chart)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let sol = run_day08_p1();
        println!("Day 8 part 1 solution is: {sol}");
    } else {
        let sol = run_day08_p2();
        println!("Day 8 part 2 solution is: {sol}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        LLR

        AAA = (BBB, BBB)\n\
        BBB = (AAA, ZZZ)\n\
        ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_2: &str = "\
        LR

        11A = (11B, XXX)\n\
        11B = (XXX, 11Z)\n\
        11Z = (11B, XXX)\n\
        22A = (22B, XXX)\n\
        22B = (22C, 22C)\n\
        22C = (22Z, 22Z)\n\
        22Z = (22B, 22B)\n\
        XXX = (XXX, XXX)";

    #[test]
    fn test_day08_p1_example() {
        assert_eq!(day08_p1(EXAMPLE), 6)
    }

    #[test]
    fn test_day08_p2_example() {
        assert_eq!(day08_p2(EXAMPLE_2), 6)
    }

    #[test]
    fn test_day08_p1() {
        assert_eq!(run_day08_p1(), 12737)
    }

    #[test]
    fn test_day08_p2() {
        assert_eq!(run_day08_p2(), 9_064_949_303_801)
    }
}
