//! Day 12: Hot Springs
use rayon::prelude::*;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
enum SpringType {
    Working(u8),
    Broken(u8),
    Unknown(u8),
}
use SpringType::*;

#[derive(Debug)]
struct SpringLine {
    springs: Vec<SpringType>,
    counts: Vec<u64>,
}

impl SpringLine {
    fn new(springs: Vec<char>, counts: Vec<u64>) -> Self {
        let mut compressed = vec![];
        let mut last = ' ';
        let mut count = 1;
        for c in springs.into_iter().chain(std::iter::once(' ')) {
            if c == last {
                count += 1;
            } else if last != ' ' {
                let sprg = match last {
                    '#' => Broken(count),
                    '.' => Working(count),
                    '?' => Unknown(count),
                    _ => panic!("Unknown spring character {:?}", c),
                };
                compressed.push(sprg);
                count = 1
            }
            last = c;
        }
        Self {
            springs: compressed,
            counts: counts,
        }
    }

    fn variants(&self) -> u64 {
        let mut groups = vec![];
        let mut group = vec![];
        for sprg in self.springs.iter() {
            match sprg {
                Working(_) => {
                    if !group.is_empty() {
                        groups.push(group);
                        group = vec![];
                    }
                }
                other => group.push(*other),
            }
        }
        if !group.is_empty() {
            groups.push(group);
        }
        assert!(!groups.is_empty());
        // dbg!(&groups);

        if groups.len() == 1 {
            return variants(
                convert(&groups.into_iter().next().unwrap()),
                self.counts.iter().map(|&x| Into::into(x)).collect(),
            );
        }
        let capacities: Vec<u64> = groups
            .iter()
            .map(|group| {
                group
                    .iter()
                    .map(|spring| match spring {
                        Working(count) => *count as u64,
                        Broken(count) => *count as u64,
                        Unknown(count) => *count as u64,
                    })
                    .sum()
            })
            .collect();
        let minimums: Vec<u64> = groups
            .iter()
            .map(|group| {
                group
                    .iter()
                    .map(|spring| match spring {
                        Broken(count) => *count as u64,
                        _ => 0,
                    })
                    .sum()
            })
            .collect();
        // dbg!(&groups);
        // dbg!(&capacities);
        generate_partitions(&self.counts, &capacities, &minimums, groups.len())
            .into_iter()
            .map(|counts| {
                // let tmp: Vec<(_, _)> = groups.iter().zip(counts.iter()).collect();
                // dbg!(tmp);

                groups
                    .iter()
                    .zip(counts)
                    .map(|(group, count)| {
                        // dbg!(&group, &count);
                        let res = variants(convert(group), count);
                        // dbg!(res);
                        res
                    })
                    .product::<u64>()
            })
            .sum()
    }
}

fn generate_partitions(
    slice: &[u64],
    capacities: &[u64],
    minimums: &[u64],
    pieces: usize,
) -> Vec<Vec<Vec<u64>>> {
    if pieces == 0 {
        panic!("called generate partitions with pieces=0 and {:?}", slice);
    }
    if pieces == 1 {
        let cur = slice.to_vec();
        let need = {
            if cur.is_empty() {
                0
            } else {
                (cur.len() as u64 - 1) + cur.iter().sum::<u64>()
            }
        };
        if capacities[0] < need || need < minimums[0] {
            return vec![];
        }
        return vec![vec![cur]];
    }
    let mut results = vec![];
    for split in 0..=slice.len() {
        let cur = slice[..split].to_vec();
        let need = {
            if cur.is_empty() {
                0
            } else {
                (cur.len() as u64 - 1) + cur.iter().sum::<u64>()
            }
        };
        if capacities[0] < need || need < minimums[0] {
            continue;
        }
        let part = generate_partitions(
            &slice[split..],
            &capacities[1..],
            &minimums[1..],
            pieces - 1,
        );
        for p in part.into_iter() {
            let mut cur2 = vec![cur.clone()];
            cur2.extend(p);
            results.push(cur2);
        }
    }
    results
}

fn convert(springs: &Vec<SpringType>) -> Vec<char> {
    springs
        .into_iter()
        .flat_map(|spring| match spring {
            Working(count) => vec!['.'; *count as usize],
            Broken(count) => vec!['#'; *count as usize],
            Unknown(count) => vec!['?'; *count as usize],
        })
        .collect()
}

fn validate(springs: &Vec<char>, counts: &Vec<u64>) -> bool {
    let mut ingroup = false;
    let mut cur_count: u64 = 0;
    let mut counts = counts.iter();
    for c in springs.iter() {
        if *c == '#' {
            cur_count += 1;
            ingroup = true;
        } else if *c == '.' {
            if ingroup {
                ingroup = false;
                if cur_count != *counts.next().unwrap_or(&0) {
                    return false;
                }
                cur_count = 0;
            }
        } else if *c == '?' {
            if ingroup && cur_count > *counts.next().unwrap_or(&0) {
                return false;
            }
            return true;
        } else {
            panic!("Encounter non-spring in validate; {:?}", c);
        }
    }
    if ingroup {
        if cur_count != *counts.next().unwrap_or(&0) {
            return false;
        }
    }
    counts.next().is_none()
}

fn variants(mut springs: Vec<char>, counts: Vec<u64>) -> u64 {
    if counts.is_empty() {
        if springs.contains(&'#') {
            return 0;
        } else {
            // dbg!(springs);
            return 1;
        }
    }
    if !validate(&springs, &counts) {
        return 0;
    }
    let indices: Vec<usize> = springs
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == '?')
        .map(|(ii, _)| ii)
        .collect();

    if indices.is_empty() {
        return 1;
    }
    variants_inner(
        {
            springs[indices[0]] = '#';
            springs.clone()
        },
        &counts,
        &indices[1..],
    ) + variants_inner(
        {
            springs[indices[0]] = '.';
            springs.clone()
        },
        &counts,
        &indices[1..],
    )
}

fn variants_inner(mut springs: Vec<char>, counts: &Vec<u64>, indices: &[usize]) -> u64 {
    if !validate(&springs, counts) {
        return 0;
    }
    if indices.is_empty() {
        return 1;
    }

    variants_inner(
        {
            springs[indices[0]] = '#';
            springs.clone()
        },
        counts,
        &indices[1..],
    ) + variants_inner(
        {
            springs[indices[0]] = '.';
            springs.clone()
        },
        counts,
        &indices[1..],
    )
}

fn day12_p1(data: &str) -> u64 {
    let data: Vec<&str> = data.lines().collect();
    data.par_iter()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').unwrap();
            let num = SpringLine::new(
                springs.chars().collect(),
                counts
                    .split(',')
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
            )
            .variants();
            // println!("{}", num);
            num
        })
        .sum()
}

fn day12_p2(data: &str) -> u64 {
    let data: Vec<&str> = data.lines().collect();
    data.par_iter()
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
            let num = SpringLine::new(springs, counts).variants();
            println!("{}", num);
            num
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

    #[test]
    fn test_day12_p2_example2() {
        assert_eq!(day12_p2("?###???????? 3,2,1"), 506250)
    }

    // #[test]
    // fn test_day12_p2_example3() {
    //     assert_eq!(day12_p2("????.######..#####. 1,6,5"), 2500)
    // }

    // #[test]
    // fn test_day12_p2_example4() {
    //     assert_eq!(day12_p2("????.#...#... 4,1,1"), 16)
    // }

    #[test]
    fn test_day12_p2_example() {
        assert_eq!(day12_p2(EXAMPLE), 525152)
    }

    // #[test]
    // fn test_day12_p2_example() {
    //     // "?????.???..????????.???..????????.???..????????.???..????????.???..?? 1,2,2,1,1,2,2,1,1,2,2,1,1,2,2,1,1,2,2,1"
    //     assert_eq!(day12_p2("?????.???..?? 1,2,2,1"), 20)
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
