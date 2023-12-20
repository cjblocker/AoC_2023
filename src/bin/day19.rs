//! Day 19: Aplenty
use regex::Regex;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug)]
struct Gizmo {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl From<&str> for Gizmo {
    fn from(data: &str) -> Self {
        dbg!(data);
        let re = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}$").unwrap();
        let captures = re.captures(data.trim()).unwrap();
        Self {
            x: captures[1].parse().unwrap(),
            m: captures[2].parse().unwrap(),
            a: captures[3].parse().unwrap(),
            s: captures[4].parse().unwrap(),
        }
    }
}

fn day19_p1(data: &str) -> usize {
    let (rules, gizmos) = data.split_once("\n\n").unwrap();
    let gizmos: Vec<Gizmo> = gizmos.lines().map(Gizmo::from).collect();
    dbg!(gizmos);
    1
}

fn day19_p2(data: &str) -> usize {
    0
}

pub fn run_day19_p1() -> usize {
    let filename = "data/day_19.txt";
    let data = read_to_string(filename).unwrap();
    day19_p1(&data)
}

pub fn run_day19_p2() -> usize {
    let filename = "data/day_19.txt";
    let data = read_to_string(filename).unwrap();
    day19_p2(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let now = Instant::now();
        let sol = run_day19_p1();
        let elapsed = now.elapsed().as_millis();
        println!("Day 19 part 1 solution is: {sol} in {elapsed} ms");
    } else {
        let now = Instant::now();
        let sol = run_day19_p2();
        let elapsed = now.elapsed().as_millis();
        println!("Day 19 part 2 solution is: {sol} in {elapsed} ms");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        px{a<2006:qkq,m>2090:A,rfg}\n\
        pv{a>1716:R,A}\n\
        lnx{m>1548:A,A}\n\
        rfg{s<537:gd,x>2440:R,A}\n\
        qs{s>3448:A,lnx}\n\
        qkq{x<1416:A,crn}\n\
        crn{x>2662:A,R}\n\
        in{s<1351:px,qqz}\n\
        qqz{s>2770:qs,m<1801:hdj,R}\n\
        gd{a>3333:R,R}\n\
        hdj{m>838:A,pv}\n\
        \n\
        {x=787,m=2655,a=1222,s=2876}\n\
        {x=1679,m=44,a=2067,s=496}\n\
        {x=2036,m=264,a=79,s=2244}\n\
        {x=2461,m=1339,a=466,s=291}\n\
        {x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_day19_p1_example() {
        assert_eq!(day19_p1(EXAMPLE), 19114);
    }

    #[test]
    #[ignore]
    fn test_day19_p2_example() {
        assert_eq!(day19_p2(EXAMPLE), 0)
    }

    #[test]
    #[ignore]
    fn test_day19_p1() {
        assert_eq!(run_day19_p1(), 0);
    }

    #[test]
    #[ignore]
    fn test_day19_p2() {
        assert_eq!(run_day19_p2(), 0);
    }
}
