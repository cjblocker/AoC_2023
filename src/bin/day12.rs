//! Day 12: Hot Springs
use std::env;
use std::fs::read_to_string;

fn validate(springs: &Vec<char>, counts: &Vec<u64>) -> bool {
    let mut actual_count = vec![];
    let mut ingroup = false;
    let mut cur_count: u64 = 0;
    for c in springs.iter() {
        if *c == '#' {
            cur_count += 1;
            ingroup = true;
        } else if *c == '.' {
            if ingroup {
                ingroup = false;
                actual_count.push(cur_count);
                cur_count = 0;
            }
        } else {
            panic!("Encounter non-spring in validate; {:?}", c);
        }
    }
    if ingroup {
        actual_count.push(cur_count);
    }
    actual_count == counts.as_slice()
}

fn variants(mut springs: Vec<char>, counts: Vec<u64>) -> u64 {
    let indices: Vec<usize> = springs
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == '?')
        .map(|(ii, _)| ii)
        .collect();
    let ncount = indices.len();
    dbg!(&ncount);
    let total_variations: u128 = 1u128 << (ncount); // 2^(ncount)
    (0..total_variations)
        .map(|x| {
            for (ii, index) in indices.iter().enumerate() {
                if (x >> (ii as u8)) & 1 == 1 {
                    springs[*index] = '#'
                } else {
                    springs[*index] = '.'
                }
            }
            springs.clone()
        })
        .filter(|spring_var| validate(&spring_var, &counts))
        .count() as u64
}

fn day12_p1(data: &str) -> u64 {
    data.lines()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').unwrap();
            variants(
                springs.chars().collect(),
                counts
                    .split(',')
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
            )
        })
        .sum()
}

fn day12_p2(data: &str) -> u64 {
    data.lines()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').unwrap();
            let mut springs: Vec<_> = springs.chars().collect();
            let mut counts: Vec<_> = counts
                .split(',')
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            let n = springs.len();
            let m = counts.len();
            for _ in 0..4 {
                springs.push('?');
                springs.extend_from_within(..n);
                counts.extend_from_within(..m);
            }
            variants(springs, counts)
        })
        .sum()
}

pub fn run_day12_p1() -> u64 {
    let filename = "data/day_12.txt";
    let data = read_to_string(filename).unwrap();
    day12_p1(&data)
}

pub fn run_day12_p2() -> u64 {
    let filename = "data/day_12.txt";
    let data = read_to_string(filename).unwrap();
    day12_p2(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let sol = run_day12_p1();
        println!("Day 12 part 1 solution is: {sol}");
    } else {
        let sol = run_day12_p2();
        println!("Day 12 part 2 solution is: {sol}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        ???.### 1,1,3\n\
        .??..??...?##. 1,1,3\n\
        ?#?#?#?#?#?#?#? 1,3,1,6\n\
        ????.#...#... 4,1,1\n\
        ????.######..#####. 1,6,5\n\
        ?###???????? 3,2,1";

    #[test]
    fn test_day12_p1_example() {
        assert_eq!(day12_p1(EXAMPLE), 21)
    }

    // #[test]
    // fn test_day12_p2_example() {
    //     assert_eq!(day12_p2(EXAMPLE), 525152)
    // }

    #[test]
    fn test_day12_p1() {
        assert_eq!(run_day12_p1(), 7402)
    }

    // #[test]
    // fn test_day12_p2() {
    //     assert_eq!(run_day12_p2(), 0)
    // }
}
