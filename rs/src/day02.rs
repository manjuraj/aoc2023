use core::fmt;
use std::str::FromStr;

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Default)]
struct Red(usize);

impl FromStr for Red {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        static RED_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r#"(?P<num>\d+) red"#).expect("failed to compile regex"));
        let caps = RED_REGEX
            .captures(s)
            .ok_or(anyhow::anyhow!(format!("failed to parse: {}", s)))?;
        let num = caps
            .name("num")
            .and_then(|n| n.as_str().parse::<usize>().ok())
            .ok_or(anyhow::anyhow!(format!("failed to parse: {}", s)))?;
        Ok(Red(num))
    }
}

impl fmt::Display for Red {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} red", self.0)
    }
}

#[derive(Debug, Default)]
struct Green(usize);

impl FromStr for Green {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        static GREEN_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r#"(?P<num>\d+) green"#).expect("failed to compile regex"));
        let caps = GREEN_REGEX
            .captures(s)
            .ok_or(anyhow::anyhow!(format!("failed to parse: {}", s)))?;
        let num = caps
            .name("num")
            .and_then(|n| n.as_str().parse::<usize>().ok())
            .ok_or(anyhow::anyhow!(format!("failed to parse: {}", s)))?;
        Ok(Green(num))
    }
}

impl fmt::Display for Green {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} green", self.0)
    }
}

#[derive(Debug, Default)]
struct Blue(usize);

impl FromStr for Blue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        static BLUE_REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r#"(?P<num>\d+) blue"#).expect("failed to compile regex"));
        let caps = BLUE_REGEX
            .captures(s)
            .ok_or(anyhow::anyhow!(format!("failed to parse: {}", s)))?;
        let num = caps
            .name("num")
            .and_then(|n| n.as_str().parse::<usize>().ok())
            .ok_or(anyhow::anyhow!(format!("failed to parse: {}", s)))?;
        Ok(Blue(num))
    }
}

impl fmt::Display for Blue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} blue", self.0)
    }
}

#[derive(Debug, Default)]
struct Bag(Red, Green, Blue);

impl FromStr for Bag {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut bag = Bag(Red(0), Green(0), Blue(0));
        for part in s.split(", ") {
            if let Ok(red) = part.parse::<Red>() {
                bag.0 = red;
            } else if let Ok(green) = part.parse::<Green>() {
                bag.1 = green;
            } else if let Ok(blue) = part.parse::<Blue>() {
                bag.2 = blue;
            }
        }
        Ok(bag)
    }
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.0, self.1, self.2)
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    bags: Vec<Bag>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        static GAME_REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r#"^Game (?P<id>\d+): (?P<rest>.*)"#).expect("failed to compile regex")
        });
        let caps = GAME_REGEX
            .captures(s)
            .ok_or(anyhow::anyhow!(format!("failed to parse: {}", s)))?;
        let id = caps
            .name("id")
            .and_then(|n| n.as_str().parse::<usize>().ok())
            .ok_or(anyhow::anyhow!(format!("failed to parse: {}", s)))?;
        let rest = caps
            .name("rest")
            .ok_or(anyhow::anyhow!(format!("failed to parse: {}", s)))?;
        let bags = rest
            .as_str()
            .split("; ")
            .map(|line| line.parse::<Bag>())
            .collect::<Result<Vec<_>>>()?;
        Ok(Game { id, bags })
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Game {}: ", self.id)?;
        for bag in &self.bags {
            write!(f, "{}; ", bag)?;
        }
        Ok(())
    }
}

impl Game {
    fn is_possible(&self, Bag(Red(ar), Green(ag), Blue(ab)): &Bag) -> bool {
        self.bags.iter().all(|bag| {
            let Bag(Red(r), Green(g), Blue(b)) = bag;
            *r <= *ar && *g <= *ag && *b <= *ab
        })
    }

    fn power(&self) -> usize {
        let Bag(Red(r), Green(g), Blue(b)) = self.bags.iter().fold(
            Bag::default(),
            |Bag(Red(ar), Green(ag), Blue(ab)), Bag(Red(r), Green(g), Blue(b))| {
                Bag(Red(ar.max(*r)), Green(ag.max(*g)), Blue(ab.max(*b)))
            },
        );
        r * g * b
    }
}

#[derive(Debug)]
struct Games(Vec<Game>);

impl FromStr for Games {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let games = s
            .lines()
            .map(|line| line.parse::<Game>())
            .collect::<Result<Vec<_>>>()?;
        Ok(Games(games))
    }
}

impl fmt::Display for Games {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for game in &self.0 {
            writeln!(f, "{} power = {}", game, game.power())?;
        }
        Ok(())
    }
}

impl Games {
    fn possible_game_ids(&self, actual: &Bag) -> Vec<usize> {
        self.0
            .iter()
            .filter(|game| game.is_possible(actual))
            .map(|game| game.id)
            .collect()
    }

    fn sum_of_possible_game_ids(&self, actual: &Bag) -> usize {
        self.possible_game_ids(actual).iter().sum()
    }

    fn sum_of_power(&self) -> usize {
        self.0.iter().map(|game| game.power()).sum()
    }
}

pub fn part1_and_part2() -> Result<()> {
    let input = include_str!("../../input/day02.txt");
    let actual = Bag(Red(12), Green(13), Blue(14));
    let games = input.parse::<Games>()?;
    tracing::debug!("games: \n{}", games);
    tracing::info!(
        "[part 1] sum of possible game ids: {:?}",
        games.sum_of_possible_game_ids(&actual)
    );
    tracing::info!(
        "[part 2] sum of power of all games: {:?}",
        games.sum_of_power()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() -> Result<()> {
        let input = include_str!("../../sample/day02.txt");
        let actual = Bag(Red(12), Green(13), Blue(14));
        let games = input.parse::<Games>()?;
        assert_eq!(games.sum_of_possible_game_ids(&actual), 8);
        assert_eq!(games.sum_of_power(), 2286);
        Ok(())
    }
}
