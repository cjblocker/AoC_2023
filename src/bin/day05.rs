use itertools::Itertools;
use std::env;
use std::fs::read_to_string;
use std::ops::Range;

#[derive(Debug)]
struct AlmanacMap {
    pub data: Vec<(Range<u64>, Range<u64>)>,
}

impl AlmanacMap {
    fn parse(almanac: &str) -> Self {
        Self {
            data: almanac
                .lines()
                .filter(|&x| !(x.trim().is_empty() || x.contains("map:")))
                .map(|line| {
                    let numbers: Vec<u64> = line
                        .trim()
                        .split(' ')
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect();
                    let [dest_start, src_start, length] = numbers[0..3] else {
                        panic!("Line parsing failed")
                    };
                    (
                        src_start..(src_start + length),
                        dest_start..(dest_start + length),
                    )
                })
                .collect(),
        }
    }

    fn map(&self, source: u64) -> u64 {
        for (src_range, dest_range) in &self.data {
            if src_range.contains(&source) {
                return (source - src_range.start) + dest_range.start;
            }
        }
        return source;
    }

    fn map_range(&self, source: Range<u64>) -> Vec<Range<u64>> {
        let covering_ranges: Vec<_> = self
            .data
            .iter()
            .filter(|&r| ranges_overlap(&r.0, &source))
            .collect();
        let mut src_ranges = vec![source.clone()];
        let mut dest_ranges = vec![];
        'outer: while !src_ranges.is_empty() {
            let curr_range = src_ranges.pop().unwrap();
            for map_range in covering_ranges.iter() {
                if ranges_overlap(&map_range.0, &curr_range) {
                    if curr_range.start >= map_range.0.start && curr_range.end <= map_range.0.end {
                        // curr_range is completely inside
                        let dest_start = (curr_range.start - map_range.0.start) + map_range.1.start;
                        let dest_end = (curr_range.end - map_range.0.start) + map_range.1.start;
                        dest_ranges.push(dest_start..dest_end);
                    } else if map_range.0.start >= curr_range.start
                        && map_range.0.end <= curr_range.end
                    {
                        // map_range.0 is in the middle of curr_range, so we divide into three pieces
                        dest_ranges.push(map_range.1.clone());
                        src_ranges.push(curr_range.start..map_range.0.start);
                        src_ranges.push(map_range.0.end..curr_range.end);
                    } else if map_range.0.start > curr_range.start
                        && map_range.0.end > curr_range.end
                    {
                        // map_range.0 overlap the back half
                        src_ranges.push(curr_range.start..map_range.0.start);
                        let dest_end = (curr_range.end - map_range.0.start) + map_range.1.start;
                        dest_ranges.push(map_range.1.start..dest_end);
                    } else if curr_range.start > map_range.0.start
                        && curr_range.end > map_range.0.end
                    {
                        // map_range.0 overlap the front half
                        src_ranges.push(map_range.0.end..curr_range.end);
                        let dest_start = (curr_range.start - map_range.0.start) + map_range.1.start;
                        dest_ranges.push(dest_start..map_range.1.end)
                    }
                    continue 'outer;
                }
            }
            // none of the maps overlapped this one
            if !curr_range.is_empty() {
                dest_ranges.push(curr_range);
            }
        }
        dest_ranges
    }
}

fn ranges_overlap(a: &Range<u64>, b: &Range<u64>) -> bool {
    !((a.end <= b.start) || (a.start >= b.end) || a.is_empty() || b.is_empty())
}

fn day05_p1(almanac: &str) -> u64 {
    // Get the list of seeds
    let (seeds, maps) = almanac.split_once("\n\n").unwrap();
    let (_, seeds) = seeds.split_once(':').unwrap();
    let seeds: Vec<u64> = seeds
        .trim()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    // Apply each almanac map in sequence
    let maps = maps.split("\n\n").map(AlmanacMap::parse);
    let mut values = seeds;
    for map in maps {
        values = values.into_iter().map(|v| map.map(v)).collect();
    }
    // return minimum value
    values.into_iter().min().unwrap()
}

fn day05_p2(almanac: &str) -> u64 {
    // Get the list of seeds
    let (seeds, maps) = almanac.split_once("\n\n").unwrap();
    let (_, seeds) = seeds.split_once(':').unwrap();
    let seeds: Vec<Range<u64>> = seeds
        .trim()
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .tuples()
        .map(|(a, b)| a..(a + b))
        .collect();

    // Apply each almanac map in sequence
    let maps = maps.split("\n\n").map(AlmanacMap::parse);
    let mut values = seeds;
    for map in maps {
        values = values.into_iter().flat_map(|v| map.map_range(v)).collect();
    }
    // return minimum value
    values.into_iter().map(|x| x.start).min().unwrap()
}

pub fn run_day05_p1() -> u64 {
    let filename = "data/day_05.txt";
    let almanac = read_to_string(filename).unwrap();
    day05_p1(&almanac)
}

pub fn run_day05_p2() -> u64 {
    let filename = "data/day_05.txt";
    let almanac = read_to_string(filename).unwrap();
    day05_p2(&almanac)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let sol = run_day05_p1();
        println!("Day 5 part 1 solution is: {sol}");
    } else {
        let sol = run_day05_p2();
        println!("Day 5 part 2 solution is: {sol}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        seeds: 79 14 55 13

        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48

        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15

        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4

        water-to-light map:\n\
        88 18 7\n\
        18 25 70

        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13

        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69

        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4";

    #[test]
    fn test_day05_p1_example() {
        assert_eq!(day05_p1(EXAMPLE), 35)
    }

    #[test]
    fn test_day05_p2_example() {
        assert_eq!(day05_p2(EXAMPLE), 46)
    }

    #[test]
    fn test_day05_p1() {
        assert_eq!(run_day05_p1(), 57075758)
    }

    #[test]
    fn test_day05_p2() {
        assert_eq!(run_day05_p2(), 31161857)
    }

    #[test]
    fn test_ranges_overlap() {
        assert!(ranges_overlap(&(0..5), &(4..6)));
        assert!(ranges_overlap(&(5..6), &(4..6)));
        assert!(ranges_overlap(&(5..6), &(0..9)));
        assert!(ranges_overlap(&(1..16), &(5..9)));

        assert!(!ranges_overlap(&(1..16), &(16..29)));
        assert!(!ranges_overlap(&(1..1), &(1..5)));
        assert!(!ranges_overlap(&(1..1), &(0..5)));
        assert!(!ranges_overlap(&(1..1), &(0..1)));
        assert!(!ranges_overlap(&(1..7), &(10..11)));
    }
}
