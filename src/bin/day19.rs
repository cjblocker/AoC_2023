//! Day 19: Aplenty
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug)]
enum Operator {
    LessThan(u32),
    GreaterThan(u32),
}

#[derive(Debug)]
enum Condition {
    X(Operator),
    M(Operator),
    A(Operator),
    S(Operator),
    True,
}

#[derive(Debug)]
struct Conditional {
    condition: Condition,
    result: String,
}

impl From<&str> for Conditional {
    fn from(data: &str) -> Self {
        // s>2770:qs
        if !data.contains(':') {
            return Self {
                condition: Condition::True,
                result: data.to_string(),
            };
        }
        let re = Regex::new(r"([x,m,a,s])([>,<])([0-9]+):([a-zA-Z]+)$").unwrap();
        let captures = re.captures(data.trim()).unwrap();
        let operand = captures[3].parse().unwrap();
        let operator = match &captures[2] {
            ">" => Operator::GreaterThan(operand),
            "<" => Operator::LessThan(operand),
            _ => unreachable!(),
        };
        let condition = match &captures[1] {
            "x" => Condition::X(operator),
            "m" => Condition::M(operator),
            "a" => Condition::A(operator),
            "s" => Condition::S(operator),
            _ => unreachable!(),
        };
        Self {
            condition,
            result: captures[4].to_string(),
        }
    }
}

impl Conditional {
    fn eval(&self, gizmo: &Gizmo) -> Option<String> {
        let cond = match self.condition {
            Condition::X(Operator::LessThan(val)) => gizmo.x < val,
            Condition::X(Operator::GreaterThan(val)) => gizmo.x > val,
            Condition::M(Operator::LessThan(val)) => gizmo.m < val,
            Condition::M(Operator::GreaterThan(val)) => gizmo.m > val,
            Condition::A(Operator::LessThan(val)) => gizmo.a < val,
            Condition::A(Operator::GreaterThan(val)) => gizmo.a > val,
            Condition::S(Operator::LessThan(val)) => gizmo.s < val,
            Condition::S(Operator::GreaterThan(val)) => gizmo.s > val,
            Condition::True => true,
        };
        if cond {
            Some(self.result.clone())
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Gizmo {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl From<&str> for Gizmo {
    fn from(data: &str) -> Self {
        // "{x=787,m=2655,a=1222,s=2876}"
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

impl Gizmo {
    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

fn day19_p1(data: &str) -> u32 {
    let (ruleset, gizmos) = data.split_once("\n\n").unwrap();
    let re = Regex::new(r"([a-zA-Z]+)\{(.+)\}$").unwrap();
    let ruleset: HashMap<String, Vec<Conditional>> = ruleset
        .lines()
        .map(|line| {
            let captures = re.captures(line.trim()).unwrap();
            let key = captures[1].to_string();
            let val = captures[2].split(',').map(Conditional::from).collect();
            (key, val)
        })
        .collect();
    gizmos
        .lines()
        .map(Gizmo::from)
        .filter_map(|gizmo| {
            let mut key = "in".to_string();
            while key != "A" && key != "R" {
                let rules = ruleset.get(&key).unwrap();
                key = rules.iter().find_map(|rule| rule.eval(&gizmo)).unwrap()
            }
            if key == "A" {
                Some(gizmo.sum())
            } else if key == "R" {
                None
            } else {
                unreachable!()
            }
        })
        .sum()
}

#[derive(Debug)]
struct GizmoRange {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
}

impl GizmoRange {
    fn new() -> Self {
        Self {
            x: (1, 4001),
            m: (1, 4001),
            a: (1, 4001),
            s: (1, 4001),
        }
    }

    fn is_empty(&self) -> bool {
        (self.x.1 <= self.x.0)
            || (self.m.1 <= self.m.0)
            || (self.a.1 <= self.a.0)
            || (self.s.1 <= self.s.0)
    }

    fn distinct(&self) -> u64 {
        if self.is_empty() {
            return 0;
        }
        (self.x.1 - self.x.0) as u64
            * (self.m.1 - self.m.0) as u64
            * (self.a.1 - self.a.0) as u64
            * (self.s.1 - self.s.0) as u64
    }

    fn apply_conditional(self, cond: &Conditional) -> ((String, Self), Option<Self>) {
        // this return type really shows how bad I setup part 1 for part 2
        // basically, we split the GizmoRange into possibly two GizmoRanges depending
        // on the condition. One if forward with a new 'key', and the other is an optional residual
        // (optional since one condition (True) doesn't split)
        match cond.condition {
            Condition::X(Operator::LessThan(val)) => (
                (
                    cond.result.clone(),
                    Self {
                        x: (self.x.0, u32::min(val, self.x.1)),
                        ..self
                    },
                ),
                Some(Self {
                    x: (u32::max(val, self.x.0), self.x.1),
                    ..self
                }),
            ),
            Condition::X(Operator::GreaterThan(val)) => (
                (
                    cond.result.clone(),
                    Self {
                        x: (u32::max(val + 1, self.x.0), self.x.1),
                        ..self
                    },
                ),
                Some(Self {
                    x: (self.x.0, u32::min(val + 1, self.x.1)),
                    ..self
                }),
            ),
            Condition::M(Operator::LessThan(val)) => (
                (
                    cond.result.clone(),
                    Self {
                        m: (self.m.0, u32::min(val, self.m.1)),
                        ..self
                    },
                ),
                Some(Self {
                    m: (u32::max(val, self.m.0), self.m.1),
                    ..self
                }),
            ),
            Condition::M(Operator::GreaterThan(val)) => (
                (
                    cond.result.clone(),
                    Self {
                        m: (u32::max(val + 1, self.m.0), self.m.1),
                        ..self
                    },
                ),
                Some(Self {
                    m: (self.m.0, u32::min(val + 1, self.m.1)),
                    ..self
                }),
            ),
            Condition::A(Operator::LessThan(val)) => (
                (
                    cond.result.clone(),
                    Self {
                        a: (self.a.0, u32::min(val, self.a.1)),
                        ..self
                    },
                ),
                Some(Self {
                    a: (u32::max(val, self.a.0), self.a.1),
                    ..self
                }),
            ),
            Condition::A(Operator::GreaterThan(val)) => (
                (
                    cond.result.clone(),
                    Self {
                        a: (u32::max(val + 1, self.a.0), self.a.1),
                        ..self
                    },
                ),
                Some(Self {
                    a: (self.a.0, u32::min(val + 1, self.a.1)),
                    ..self
                }),
            ),
            Condition::S(Operator::LessThan(val)) => (
                (
                    cond.result.clone(),
                    Self {
                        s: (self.s.0, u32::min(val, self.s.1)),
                        ..self
                    },
                ),
                Some(Self {
                    s: (u32::max(val, self.s.0), self.s.1),
                    ..self
                }),
            ),
            Condition::S(Operator::GreaterThan(val)) => (
                (
                    cond.result.clone(),
                    Self {
                        s: (u32::max(val + 1, self.s.0), self.s.1),
                        ..self
                    },
                ),
                Some(Self {
                    s: (self.s.0, u32::min(val + 1, self.s.1)),
                    ..self
                }),
            ),
            Condition::True => ((cond.result.clone(), self), None),
        }
    }
}

fn day19_p2(data: &str) -> u64 {
    let (ruleset, _) = data.split_once("\n\n").unwrap();
    let re = Regex::new(r"([a-zA-Z]+)\{(.+)\}$").unwrap();
    let ruleset: HashMap<String, Vec<Conditional>> = ruleset
        .lines()
        .map(|line| {
            let captures = re.captures(line.trim()).unwrap();
            let key = captures[1].to_string();
            let val = captures[2].split(',').map(Conditional::from).collect();
            (key, val)
        })
        .collect();
    let mut to_process = vec![("in".to_string(), GizmoRange::new())];
    let mut valid = vec![];
    while let Some((key, range)) = to_process.pop() {
        let rules = ruleset.get(&key).unwrap();
        rules.iter().try_fold(range, |left: GizmoRange, rule| {
            let (new, residual) = left.apply_conditional(rule);
            if !new.1.is_empty() {
                if new.0 == "A" {
                    valid.push(new.1);
                } else if new.0 != "R" {
                    to_process.push(new);
                }
            }
            residual
        });
    }
    valid.into_iter().map(|x| x.distinct()).sum()
}

pub fn run_day19_p1() -> u32 {
    let filename = "data/day_19.txt";
    let data = read_to_string(filename).unwrap();
    day19_p1(&data)
}

pub fn run_day19_p2() -> u64 {
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
    fn test_day19_p2_example() {
        assert_eq!(day19_p2(EXAMPLE), 167409079868000)
    }

    #[test]
    fn test_day19_p1() {
        assert_eq!(run_day19_p1(), 449531);
    }

    #[test]
    fn test_day19_p2() {
        assert_eq!(run_day19_p2(), 122756210763577);
    }
}
