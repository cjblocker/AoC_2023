//! Day 12: Hot Springs
use rayon::prelude::*;
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

// This one took me two days, and the code along the way was pretty ugly (still pretty ugly).
// My final solution still takes around 20 seconds to solve it.

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

fn nchoosek(n: u64, k: u64) -> u64 {
    let n = n as u128;
    let k = k as u128;
    (((n - k + 1)..=n).product::<u128>() / (1..=k).product::<u128>()) as u64
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
            counts,
        }
    }

    fn variants(&self) -> u64 {
        // The idea of this function is to first separate springs into distinct groups
        // separated by '.' (Working) springs. Each group with only Broken # and Unknown ?
        // springs is then individually processed (with `partition_variants`) and the results combined (via product).
        // When we split into groups, we also have to split the group counts into separate groups as well.
        // This is accomplished with `generate_partitions`. Some counts could feasibly belong to different
        // groups so we try each as well and add the number of variants of each together.
        let mut groups = vec![];
        let mut group = vec![];
        // break into groups
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

        // determine the capacity of each group for generate_partions
        let capacities: Vec<[u64; 2]> = groups
            .iter()
            .map(|group| {
                let max = group
                    .iter()
                    .map(|spring| match spring {
                        Working(count) => *count as u64,
                        Broken(count) => *count as u64,
                        Unknown(count) => *count as u64,
                    })
                    .sum();
                let min = group
                    .iter()
                    .map(|spring| match spring {
                        Broken(count) => *count as u64,
                        _ => 0,
                    })
                    .sum();
                [max, min]
            })
            .collect();

        generate_partitions(&self.counts, &capacities, groups.len())
            .into_iter()
            .map(|counts| {
                groups
                    .iter()
                    .zip(counts)
                    .map(|(group, count)| match group[..] {
                        [Unknown(n)] => {
                            nchoosek(n as u64 - count.iter().sum::<u64>() + 1, count.len() as u64)
                        }
                        _ => partition_variants(&convert(group), &count),
                    })
                    .product::<u64>()
            })
            .sum()
    }
}

fn generate_partitions(
    counts: &[u64],
    capacities: &[[u64; 2]],
    pieces: usize,
) -> Vec<Vec<Vec<u64>>> {
    // The purpose of this function is to take `counts` and return
    // all of the way it can be partitioned into `pieces` groups.
    // `capacities` sets the maximum and minimum of count space
    // is needed for each group so we can avoid generating partitions of `counts`
    // that won't work. `capacities` should both be `pieces` long.
    if pieces == 0 {
        panic!("called generate partitions with pieces=0 and {:?}", counts);
    }
    if pieces == 1 {
        // base case for the recursion
        let cur = counts.to_vec();
        let need = {
            if cur.is_empty() {
                0
            } else {
                (cur.len() as u64 - 1) + cur.iter().sum::<u64>()
            }
        };
        if capacities[0][0] < need || need < capacities[0][1] {
            return vec![];
        }
        return vec![vec![cur]];
    }
    let mut results = vec![];
    for split in 0..=counts.len() {
        let cur = counts[..split].to_vec();
        let need = {
            if cur.is_empty() {
                0
            } else {
                (cur.len() as u64 - 1) + cur.iter().sum::<u64>()
            }
        };
        if capacities[0][0] < need || need < capacities[0][1] {
            continue;
        }
        let part = generate_partitions(&counts[split..], &capacities[1..], pieces - 1);
        for p in part.into_iter() {
            let mut cur2 = vec![cur.clone()];
            cur2.extend(p);
            results.push(cur2);
        }
    }
    results
}

fn convert(springs: &[SpringType]) -> Vec<char> {
    springs
        .iter()
        .flat_map(|spring| match spring {
            Working(count) => vec!['.'; *count as usize],
            Broken(count) => vec!['#'; *count as usize],
            Unknown(count) => vec!['?'; *count as usize],
        })
        .collect()
}

fn partition_variants(springs: &[char], counts: &[u64]) -> u64 {
    // This function assumes `springs` contains only '#' and '?'
    // as we would have already partitioned out earlier '.'
    if counts.is_empty() {
        if springs.contains(&'#') {
            return 0;
        } else {
            return 1;
        }
    } else if counts.len() == 1 {
        // recursion base case
        // handle the single group case
        let first_hash = springs.iter().position(|c| *c == '#');
        let last_hash = springs.iter().rposition(|c| *c == '#');

        return match (first_hash, last_hash) {
            (Some(first), Some(last)) => {
                if last - first >= counts[0] as usize {
                    0
                } else {
                    let left = (last as u64 + 1).saturating_sub(counts[0]);
                    let right = u64::min(first as u64, (springs.len() as u64) - counts[0]);
                    right - left + 1
                }
            },
            _ /* None, None */ => (springs.len() as u64) - counts[0] + 1
        };
    }

    let cur_count = counts[0] as usize;
    let cur_group: &[char] = &springs[..cur_count];
    let first_hash = cur_group
        .iter()
        .position(|c| *c == '#')
        .unwrap_or(cur_count);

    let next_counts = &counts[1..];
    let next_space = next_counts.iter().sum::<u64>() as usize + (next_counts.len() - 1);
    let end_stop = usize::min(springs.len() - next_space, first_hash + cur_count + 1);

    let mut sum = 0;
    if (springs[(cur_count) + 1..].len() >= (next_space + cur_count + 1))
        && (springs[..(cur_count + 1)].iter().all(|&c| c == '?'))
    {
        // handle case where everything up to cur_count is '.'
        sum += partition_variants(&springs[cur_count + 1..], counts);
    }
    for split in cur_count..end_stop {
        if springs[split] != '?' {
            continue;
        }
        // I tested with switching over to `nchoosek` when the rest of the slice is ?
        // but it actually slowed it down in some cases and was negligible overall.
        let next_groups = &springs[(split + 1)..];
        sum += partition_variants(next_groups, next_counts);
    }
    sum
}

fn day12_p1(data: &str) -> u64 {
    let data: Vec<&str> = data.lines().collect();
    data.par_iter()
        .map(|line| {
            let (springs, counts) = line.split_once(' ').unwrap();
            SpringLine::new(
                springs.chars().collect(),
                counts
                    .split(',')
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect(),
            )
            .variants()
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
            SpringLine::new(springs, counts).variants()
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
        let now = Instant::now();
        let sol = run_day12_p1();
        let elapsed = now.elapsed().as_secs();
        println!("Day 12 part 1 solution is: {sol} in {elapsed} seconds");
    } else {
        let now = Instant::now();
        let sol = run_day12_p2();
        let elapsed = now.elapsed().as_secs();
        println!("Day 12 part 2 solution is: {sol} in {elapsed} seconds");
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
        assert_eq!(day12_p1(EXAMPLE), 21);
    }

    #[test]
    fn test_day12_p2_example() {
        assert_eq!(day12_p2(EXAMPLE), 525152)
    }

    #[test]
    fn test_day12_p1() {
        assert_eq!(run_day12_p1(), 7402);
    }

    #[test]
    #[ignore]
    fn test_day12_p2() {
        assert_eq!(run_day12_p2(), 3_384_337_640_277);
    }

    #[test]
    fn test_nchoosek() {
        assert_eq!(nchoosek(5, 3), 10);
        assert_eq!(nchoosek(9, 3), 84);
        assert_eq!(nchoosek(11, 4), 330);
        assert_eq!(nchoosek(13, 2), 78);
        assert_eq!(nchoosek(17, 10), 19448);
    }

    #[test]
    fn test_nchoosek_works() {
        assert_eq!(day12_p1("?????????? 3,1,1"), nchoosek(10 - 5 + 1, 3));
    }

    #[test]
    fn test_nchoosek_works2() {
        assert_eq!(
            day12_p1("?###????????????#?? 6,3,7"),
            nchoosek(19 - 16 + 1, 3)
        );
    }

    #[test]
    fn test_partition_variants_size1() {
        assert_eq!(
            partition_variants(&['?', '?', '?', '#', '#', '?', '?'], &[4]),
            3
        );
        assert_eq!(
            partition_variants(&['#', '#', '?', '?', '?', '?', '?'], &[4]),
            1
        );
        assert_eq!(
            partition_variants(&['?', '#', '#', '?', '?', '?', '?'], &[4]),
            2
        );
        assert_eq!(
            partition_variants(&['#', '#', '#', '?', '?', '?', '#'], &[4]),
            0
        );
    }
}
