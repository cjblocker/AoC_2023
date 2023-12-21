//! Day 20: Pulse Propagation
// Feels like a good day to try out dynamic dispatch
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs::read_to_string;
use std::ops::Not;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    High,
    Low,
}

impl Not for Pulse {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }
}

#[derive(Debug, Clone)]
struct Signal {
    state: Pulse,
    src: String,
    dst: String,
}

impl Signal {
    fn to_string(&self) -> String {
        match self.state {
            Pulse::High => self.src.to_owned() + " -high-> " + &self.dst,
            Pulse::Low => self.src.to_owned() + " -low-> " + &self.dst,
        }
    }
}

trait Module {
    fn drive_wires(&mut self, sig: Signal) -> Vec<Signal>;

    fn in_reset_state(&self) -> bool;

    fn add_src(&mut self, _src: String) {
        ()
    }
}

#[derive(Debug)]
struct FlipFlop {
    id: String,
    state: Pulse,
    dst: Vec<String>,
}

impl Module for FlipFlop {
    fn drive_wires(&mut self, sig: Signal) -> Vec<Signal> {
        match sig.state {
            Pulse::High => vec![], // do nothing
            Pulse::Low => {
                self.state = !self.state;
                self.dst
                    .iter()
                    .map(|dst| Signal {
                        state: self.state,
                        src: self.id.clone(),
                        dst: dst.clone(),
                    })
                    .collect()
            }
        }
    }

    fn in_reset_state(&self) -> bool {
        self.state == Pulse::Low
    }
}

#[derive(Debug)]
struct Conjunction {
    id: String,
    state: HashMap<String, Pulse>,
    dst: Vec<String>,
}

impl Module for Conjunction {
    fn drive_wires(&mut self, sig: Signal) -> Vec<Signal> {
        *self.state.get_mut(&sig.src).expect("Unknown key") = sig.state;
        if self.state.values().all(|&state| state == Pulse::High) {
            self.dst
                .iter()
                .map(|dst| Signal {
                    state: Pulse::Low,
                    src: self.id.clone(),
                    dst: dst.clone(),
                })
                .collect()
        } else {
            self.dst
                .iter()
                .map(|dst| Signal {
                    state: Pulse::High,
                    src: self.id.clone(),
                    dst: dst.clone(),
                })
                .collect()
        }
    }

    fn in_reset_state(&self) -> bool {
        self.state.values().all(|&state| state == Pulse::Low)
    }

    fn add_src(&mut self, src: String) {
        self.state.insert(src, Pulse::Low);
    }
}

#[derive(Debug)]
struct Broadcaster {
    dst: Vec<String>,
}

impl Module for Broadcaster {
    fn drive_wires(&mut self, sig: Signal) -> Vec<Signal> {
        self.dst
            .iter()
            .map(|dst| Signal {
                state: sig.state,
                src: "broadcaster".to_string(),
                dst: dst.clone(),
            })
            .collect()
    }

    fn in_reset_state(&self) -> bool {
        true
    }
}

fn initialize_system(data: &str) -> (HashMap<String, Box<dyn Module>>, String) {
    let mut sources: HashMap<String, Vec<String>> = HashMap::new();
    let mut modules: HashMap<String, Box<dyn Module>> = data
        .lines()
        .map(|line| {
            let (ident, dest) = line.split_once(" -> ").unwrap();
            let dest: Vec<String> = dest.split(',').map(|x| x.trim().to_owned()).collect();
            if let Some(rest) = ident.strip_prefix("%") {
                dest.iter().for_each(|dst| {
                    sources
                        .entry(dst.to_string())
                        .or_insert(Vec::new())
                        .push(rest.to_string())
                });
                (
                    rest.to_string(),
                    Box::new(FlipFlop {
                        id: rest.to_string(),
                        state: Pulse::Low,
                        dst: dest,
                    }) as Box<dyn Module>,
                )
            } else if let Some(rest) = ident.strip_prefix("&") {
                dest.iter().for_each(|dst| {
                    sources
                        .entry(dst.to_string())
                        .or_insert(Vec::new())
                        .push(rest.to_string())
                });
                (
                    rest.to_string(),
                    Box::new(Conjunction {
                        id: rest.to_string(),
                        state: HashMap::new(),
                        dst: dest,
                    }) as Box<dyn Module>,
                )
            } else if let Some(_rest) = ident.strip_prefix("broadcaster") {
                dest.iter().for_each(|dst| {
                    sources
                        .entry(dst.to_string())
                        .or_insert(Vec::new())
                        .push("broadcaster".to_string())
                });
                (
                    "broadcaster".to_string(),
                    Box::new(Broadcaster { dst: dest }) as Box<dyn Module>,
                )
            } else {
                panic!("Could not parse {ident:?}");
            }
        })
        .collect();
    let mut output = "".to_string();
    sources.into_iter().for_each(|(key, srcs)| {
        // dbg!(&key);
        match modules.get_mut(&key) {
            Some(modl) => srcs.into_iter().for_each(|src| modl.add_src(src)),
            None => {
                assert_eq!(output, "");
                output = key.to_owned()
            }
        }
    });
    (modules, output)
}

fn day20_p1(data: &str) -> u32 {
    let (mut modules, output) = initialize_system(data);
    let mut pulse_counts = vec![];
    while pulse_counts.len() == 0 || modules.values().any(|modl| !modl.in_reset_state()) {
        let mut pulse_count = (0, 0);
        let mut queue = VecDeque::new();
        queue.push_back(Signal {
            state: Pulse::Low,
            src: "button".to_string(),
            dst: "broadcaster".to_string(),
        });
        while let Some(signal) = queue.pop_front() {
            // println!("{}", signal.to_string());
            match signal.state {
                Pulse::Low => pulse_count.0 += 1,
                Pulse::High => pulse_count.1 += 1,
            }
            if signal.dst != output {
                modules
                    .get_mut(&signal.dst)
                    .unwrap()
                    .drive_wires(signal)
                    .into_iter()
                    .for_each(|signal| queue.push_back(signal));
            }
        }
        dbg!(&pulse_count);
        pulse_counts.push(pulse_count);
        if pulse_counts.len() == 1000 {
            break;
        }
    }
    let cycles = pulse_counts.len();
    let (full, rem) = (1000 / cycles as u32, 1000 % cycles);
    let high_count: u32 = pulse_counts.iter().map(|x| x.1).sum::<u32>() * full
        + pulse_counts.iter().take(rem).map(|x| x.1).sum::<u32>();
    let low_count: u32 = pulse_counts.iter().map(|x| x.0).sum::<u32>() * full
        + pulse_counts.iter().take(rem).map(|x| x.0).sum::<u32>();
    dbg!(pulse_counts, low_count, high_count);
    low_count * high_count
}

fn day20_p2(data: &str) -> u64 {
    let (mut modules, output) = initialize_system(data);
    let mut cycle_count = 0;
    while cycle_count == 0 || modules.values().any(|modl| !modl.in_reset_state()) {
        let mut pulse_count = (0, 0);
        let mut queue = VecDeque::new();
        queue.push_back(Signal {
            state: Pulse::Low,
            src: "button".to_string(),
            dst: "broadcaster".to_string(),
        });
        while let Some(signal) = queue.pop_front() {
            // println!("{}", signal.to_string());
            if signal.dst != output {
                modules
                    .get_mut(&signal.dst)
                    .unwrap()
                    .drive_wires(signal)
                    .into_iter()
                    .for_each(|signal| queue.push_back(signal));
            } else {
                match signal.state {
                    Pulse::Low => pulse_count.0 += 1,
                    Pulse::High => pulse_count.1 += 1,
                }
            }
        }
        cycle_count += 1;
        // println!("{cycle_count:?}: {pulse_count:?}");
        if pulse_count == (1, 0) {
            return cycle_count;
        }
        // if cycle_count == 1000 {
        //     break;
        // }
    }
    1001
}

pub fn run_day20_p1() -> u32 {
    let filename = "data/day_20.txt";
    let data = read_to_string(filename).unwrap();
    day20_p1(&data)
}

pub fn run_day20_p2() -> u64 {
    let filename = "data/day_20.txt";
    let data = read_to_string(filename).unwrap();
    day20_p2(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let now = Instant::now();
        let sol = run_day20_p1();
        let elapsed = now.elapsed().as_millis();
        println!("Day 20 part 1 solution is: {sol} in {elapsed} ms");
    } else {
        let now = Instant::now();
        let sol = run_day20_p2();
        let elapsed = now.elapsed().as_millis();
        println!("Day 20 part 2 solution is: {sol} in {elapsed} ms");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        broadcaster -> a\n\
        %a -> inv, con\n\
        &inv -> b\n\
        %b -> con\n\
        &con -> output";

    #[test]
    fn test_day20_p1_example() {
        assert_eq!(day20_p1(EXAMPLE), 11687500);
    }

    #[test]
    fn test_day20_p1() {
        assert_eq!(run_day20_p1(), 788081152);
    }

    #[test]
    #[ignore]
    fn test_day20_p2() {
        assert_eq!(run_day20_p2(), 0);
    }
}
