use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct CubeDraw {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

struct Game {
    pub id: u32,
    pub draws: Vec<CubeDraw>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let re = Regex::new(r"Game ([0-9]+): (.+)$").unwrap();
        let captures = re.captures(line).unwrap();
        let id = &captures[1];
        let id: u32 = id.parse().unwrap();
        let draws: Vec<CubeDraw> = captures[2].split(';').map(CubeDraw::parse).collect();
        Self { id, draws }
    }

    fn possible(&self, red: u32, green: u32, blue: u32) -> bool {
        self.draws.iter().all(|x| x.possible(red, green, blue))
    }

    fn minimal(&self) -> CubeDraw {
        CubeDraw {
            red: self.draws.iter().map(|draw| draw.red).max().unwrap(),
            green: self.draws.iter().map(|draw| draw.green).max().unwrap(),
            blue: self.draws.iter().map(|draw| draw.blue).max().unwrap(),
        }
    }

    fn min_power(&self) -> u32 {
        let minimal = self.minimal();
        minimal.red * minimal.green * minimal.blue
    }
}

impl CubeDraw {
    fn parse(line: &str) -> Self {
        let mut draw = Self {
            red: 0,
            green: 0,
            blue: 0,
        };
        for cdraw in line.split(", ") {
            let cdraw: Vec<&str> = cdraw.trim().split(' ').collect();
            let num: u32 = cdraw.first().unwrap().parse().unwrap();
            let color: &str = cdraw.last().unwrap();
            match color {
                "red" => draw.red += num,
                "green" => draw.green += num,
                "blue" => draw.blue += num,
                &_ => panic!("parsed {color:?}"),
            };
        }
        draw
    }

    fn possible(&self, red: u32, green: u32, blue: u32) -> bool {
        (self.red <= red) && (self.green <= green) && (self.blue <= blue)
    }
}

pub fn day02_p1() -> u32 {
    let filename = "day_02.txt";
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(Game::parse)
        .filter(|game| game.possible(12, 13, 14))
        .map(|game| game.id)
        .sum()
}

pub fn day02_p2() -> u32 {
    let filename = "day_02.txt";
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(Game::parse)
        .map(|game| game.min_power())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_day02_p1_example() {
        let sol: u32 = EXAMPLE
            .lines()
            .map(Game::parse)
            .filter(|game| game.possible(12, 13, 14))
            .map(|game| game.id)
            .sum();
        assert_eq!(sol, 8);
    }

    #[test]
    fn test_day02_p1() {
        assert_eq!(day02_p1(), 2369)
    }

    #[test]
    fn test_day02_p2_example() {
        let sol: Vec<u32> = EXAMPLE
            .lines()
            .map(Game::parse)
            .map(|game| game.min_power())
            .collect();
        assert_eq!(sol, &[48, 12, 1560, 630, 36]);
    }

    #[test]
    fn test_day02_p2() {
        assert_eq!(day02_p2(), 66363)
    }
}
