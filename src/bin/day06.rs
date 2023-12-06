use std::env;

/// Computes the number of different amounts of whole milliseconds we could hold the boat button
/// and still break the record distance in the given amount of time.
fn day06_p1(time: Vec<u64>, dist: Vec<u64>) -> u64 {
    // The numbers (of part 1) are small enough that we can just brute force this.
    // We can think of this problem as the distance between the zeros of
    // this concave quadratic: y(x) = (T-x)x - D
    // so maybe: floor((T+sqrt(T^2 - 4D))/2) - ceil((T-sqrt(T^2 - 4D))/2)
    // but that is way more complicated than just iterating.
    time.into_iter()
        .zip(dist)
        .map(|(t, d)| (0..t).map(|x| (t - x) * x).filter(|y| y > &d).count() as u64)
        .product()
}

pub fn run_day06_p1() -> u64 {
    // The provided input is so small, it would take longer to write parsing code than
    // just pasting it here like this.
    let time = vec![60, 94, 78, 82];
    let dist = vec![475, 2138, 1015, 1650];
    day06_p1(time, dist)
}

pub fn run_day06_p2() -> u64 {
    // I feel like this was meant to be harder than part 1 because these numbers are
    // so large that it would take too long to brute force like part 1. But Rust is
    // so fast that it can check them all in no time at all. Still we could compute
    // the widths directly instead via formula. Maybe I'll try that later.
    let time = vec![60947882];
    let dist = vec![475213810151650];
    day06_p1(time, dist)
}

fn main() {
    let part1 = if let Some(arg1) = env::args().nth(1) {
        arg1.parse().unwrap_or(1) == 1
    } else {
        true
    };
    if part1 {
        let sol = run_day06_p1();
        println!("Day 6 part 1 solution is: {sol}");
    } else {
        let sol = run_day06_p2();
        println!("Day 6 part 2 solution is: {sol}");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day06_p1_example() {
        let time = vec![7, 15, 30];
        let dist = vec![9, 40, 200];
        assert_eq!(day06_p1(time, dist), 288)
    }

    #[test]
    fn test_day06_p2_example() {
        let time = vec![71530];
        let dist = vec![940200];
        assert_eq!(day06_p1(time, dist), 71503)
    }

    #[test]
    fn test_day06_p1() {
        assert_eq!(run_day06_p1(), 345015)
    }

    #[test]
    fn test_day06_p2() {
        assert_eq!(run_day06_p2(), 42588603)
    }
}
