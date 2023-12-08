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
                        left: lval.replace("(", "").trim().to_owned(),
                        right: rval.replace(")", "").trim().to_owned(),
                    },
                )
            })
            .collect();
        Self { turns, connections }
    }
}

fn day08_p1(chart: &str) -> u64 {
    let chart = Chart::from(chart);
    let Chart { turns, connections } = chart;
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

fn map_start_to_end(key: mut String, turns: &str) -> String {
    for direction in turns.chars() {
        key = match direction {
            'L' => connections.get(&key).unwrap().left.to_owned(),
            'R' => connections.get(&key).unwrap().right.to_owned(),
            _ => panic!("Unknown direction"),
        };
    }
    key
}

fn day08_p2(chart: &str) -> u64 {
    let Chart { turns, connections } = Chart::from(chart);
    let mut keys: Vec<String> = connections
        .keys()
        .filter(|key| key.as_bytes()[2] == b'A')
        .map(|key| key.to_owned())
        .collect();
    assert!(keys.len() > 0);
    dbg!(&keys);
    let mut steps = 0;
    for (step, direction) in turns.chars().cycle().enumerate() {
        keys = keys
            .into_iter()
            .map(|key| match direction {
                'L' => connections.get(&key).unwrap().left.to_owned(),
                'R' => connections.get(&key).unwrap().right.to_owned(),
                _ => panic!("Unknown direction"),
            })
            .collect();
        if keys.iter().all(|key| key.as_bytes()[2] == b'Z') {
            steps = step + 1;
            break;
        }
        let count = keys.iter().filter(|key| key.as_bytes()[2] == b'Z').count();
        if count > 1 {
            println!("{} {:?}", count, &keys);
        }
    }
    steps as u64
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
        println!("Day 7 part 1 solution is: {sol}");
    } else {
        let sol = run_day08_p2();
        println!("Day 7 part 2 solution is: {sol}");
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
        assert_eq!(day08_p2(EXAMPLE_2), 5)
    }

    #[test]
    fn test_day08_p1() {
        assert_eq!(run_day08_p1(), 12737)
    }

    // #[test]
    // fn test_day08_p2() {
    //     assert_eq!(run_day08_p2(), 252898370)
    // }
}
