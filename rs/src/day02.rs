use anyhow::Result;
use core::fmt;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Default)]
struct Color(usize, usize, usize);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Color(r, g, b) = self;
        write!(f, "{} red, {} green, {} blue", r, g, b)
    }
}

impl Color {
    fn power(&self) -> usize {
        let Color(r, g, b) = self;
        r * g * b
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Color>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Game { id, rounds } = self;
        write!(f, "Game {}: ", id)?;
        for round in rounds {
            write!(f, "{}; ", round)?;
        }
        Ok(())
    }
}

impl Game {
    fn power(&self) -> usize {
        self.rounds
            .iter()
            .fold(Color::default(), |Color(ar, ag, ab), Color(r, g, b)| {
                Color(ar.max(*r), ag.max(*g), ab.max(*b))
            })
            .power()
    }
}
#[derive(Debug)]
struct Games(Vec<Game>);

impl fmt::Display for Games {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for game in &self.0 {
            writeln!(f, "{}", game)?;
        }
        Ok(())
    }
}

impl Games {
    fn sum_of_possible_game_ids(&self) -> usize {
        static BAG: Color = Color(12, 13, 14);
        self.0
            .iter()
            .filter_map(|Game { id, rounds }| {
                rounds
                    .iter()
                    .all(|c| c.0 <= BAG.0 && c.1 <= BAG.1 && c.2 <= BAG.2)
                    .then_some(*id)
            })
            .sum()
    }

    fn sum_of_power(&self) -> usize {
        self.0.iter().map(Game::power).sum()
    }
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, (_, id, _, rounds)) = tuple((
        tag("Game "),
        parse_usize,
        tag(": "),
        separated_list1(tag("; "), parse_rounds),
    ))(input)?;
    Ok((input, Game { id, rounds }))
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse::<usize>)(input)
}

fn parse_rounds(input: &str) -> IResult<&str, Color> {
    let (input, colors) = separated_list1(tag(", "), parse_color)(input)?;
    let color = colors
        .iter()
        .fold(Color::default(), |Color(ar, ag, ab), &Color(r, g, b)| {
            Color(ar.max(r), ag.max(g), ab.max(b))
        });
    Ok((input, color))
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, (num, _, color)) = tuple((
        parse_usize,
        space1,
        alt((tag("red"), tag("green"), tag("blue"))),
    ))(input)?;
    let color = match color {
        "red" => Color(num, 0, 0),
        "green" => Color(0, num, 0),
        "blue" => Color(0, 0, num),
        _ => unreachable!(),
    };
    Ok((input, color))
}

pub fn part1_and_part2() -> Result<()> {
    let games = include_str!("../../input/day02.txt")
        .lines()
        .map(parse_game)
        .map(|res| res.map(|(_, game)| game))
        .collect::<Result<Vec<_>, _>>()?;
    let games = Games(games);
    tracing::debug!("games: \n{}", games);

    let part1 = games.sum_of_possible_game_ids();
    tracing::info!("[part 1] sum of possible game ids: {:?}", part1);
    assert_eq!(part1, 2268);

    let part2 = games.sum_of_power();
    tracing::info!("[part 2] sum of power of all games: {:?}", part2);
    assert_eq!(part2, 63542);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() -> Result<()> {
        let games = include_str!("../../sample/day02.txt")
            .lines()
            .map(parse_game)
            .map(|res| res.map(|(_, game)| game))
            .collect::<Result<Vec<_>, _>>()?;
        let games = Games(games);
        assert_eq!(games.sum_of_possible_game_ids(), 8);
        assert_eq!(games.sum_of_power(), 2286);
        Ok(())
    }
}
