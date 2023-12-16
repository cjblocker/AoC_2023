//! Day 15: Lens Library
use std::env;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Lens<'a> {
    label: &'a [u8],
    focal: u8,
}

fn holiday_hash(data: &[u8]) -> u8 {
    data.iter()
        .fold(0, |acc, &x| acc.wrapping_add(x).wrapping_mul(17))
}

fn day15_p1(data: &str) -> usize {
    data.trim()
        .split(',')
        .map(|x| x.as_bytes())
        .map(|x| holiday_hash(x) as usize)
        .sum()
}

fn day15_p2(data: &str) -> usize {
    let mut hashmap: [Vec<Lens>; 256] = [(); 256].map(|_| Vec::new());
    data.trim()
        .split(',')
        .map(|x| x.as_bytes())
        .for_each(|x| match x {
            [label @ .., b'-'] => {
                let hash = holiday_hash(label) as usize;
                let lensbox: &mut Vec<Lens> = &mut hashmap[hash];
                lensbox.retain(|x| x.label != label);
            }
            [label @ .., b'=', f] => {
                let hash = holiday_hash(label) as usize;
                let lensbox: &mut Vec<Lens> = &mut hashmap[hash];
                let f = f - b'0'; // convert ascii to number
                let mut found = false;
                for lens in lensbox.iter_mut() {
                    if lens.label == label {
                        lens.focal = f;
                        found = true;
                        break;
                    }
                }
                if !found {
                    lensbox.push(Lens { label, focal: f });
                }
            }
            _ => panic!("Found {:?}", std::str::from_utf8(x).unwrap()),
        });

    hashmap
        .iter()
        .enumerate()
        .map(|(box_num, lensbox)| {
            lensbox
                .iter()
                .enumerate()
                .map(|(i, x)| (box_num + 1) * (i + 1) * (x.focal as usize))
                .sum::<usize>()
        })
        .sum()
}

pub fn run_day15_p1() -> usize {
    let filename = "data/day_15.txt";
    let data = read_to_string(filename).unwrap();
    day15_p1(&data)
}

pub fn run_day15_p2() -> usize {
    let filename = "data/day_15.txt";
    let data = read_to_string(filename).unwrap();
    day15_p2(&data)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let now = Instant::now();
        let sol = run_day15_p1();
        let elapsed = now.elapsed().as_millis();
        println!("Day 15 part 1 solution is: {sol} in {elapsed} ms");
    } else {
        let now = Instant::now();
        let sol = run_day15_p2();
        let elapsed = now.elapsed().as_millis();
        println!("Day 15 part 2 solution is: {sol} in {elapsed} ms");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_holiday_hash() {
        assert_eq!(holiday_hash(&[b'H', b'A', b'S', b'H']), 52);
    }

    #[test]
    fn test_day15_p1_example() {
        assert_eq!(day15_p1(EXAMPLE), 1320);
    }

    #[test]
    fn test_day15_p2_example() {
        assert_eq!(day15_p2(EXAMPLE), 145)
    }

    #[test]
    fn test_day15_p1() {
        assert_eq!(run_day15_p1(), 510792);
    }

    #[test]
    fn test_day15_p2() {
        assert_eq!(run_day15_p2(), 269410);
    }
}
