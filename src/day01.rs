use std::fs::read_to_string;

pub fn day01_p1() -> u32 {
    let filename = "day_01.txt";
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>())
        .map(|line| {
            format!("{}{}", line.first().unwrap(), line.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

pub fn day01_p2() -> u32 {
    let filename = "day_01.txt";
    // let filename = "day_01_test.txt";
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| {
            line.replace("oneight", "o18t")
                .replace("twone", "t21e")
                .replace("threeight", "t38t")
                .replace("fiveight", "58t")
                .replace("sevenine", "79e")
                .replace("eightwo", "e82o")
                .replace("eighthree", "e83e")
                .replace("nineight", "n98t")
                .replace("zerone", "01e")
                .replace("one", "1")
                .replace("two", "2")
                .replace("three", "3")
                .replace("four", "4")
                .replace("five", "5")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "8")
                .replace("nine", "9")
                .replace("zero", "0")
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<Vec<_>>()
        })
        .map(|line| {
            format!("{}{}", line.first().unwrap(), line.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_day01_p1() {
        assert_eq!(day01_p1(), 55130);
    }

    #[test]
    fn test_day01_p2() {
        assert_eq!(day01_p2(), 54985);
    }
}
