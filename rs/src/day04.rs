use std::{collections::HashSet, fmt, str::FromStr};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, space0, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

/// Every scratchcard, has a
/// - unique id
/// - set of winning numbers
/// - set of numbers I have
/// - copies of scratchcards including original won
#[derive(Debug)]
struct Card {
    id: usize,
    copies: usize,
    winning_numbers: HashSet<usize>,
    my_numbers: HashSet<usize>,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let winning_numbers = self
            .winning_numbers
            .iter()
            .sorted()
            .map(|n| format!("{:>3}", n))
            .join(" ");
        let my_numbers = self
            .my_numbers
            .iter()
            .sorted()
            .map(|n| format!("{:>3}", n))
            .join(" ");
        write!(
            f,
            "Card {:3}  #{:<8}: {} | {}",
            self.id, self.copies, winning_numbers, my_numbers
        )
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, card) =
            parse_card(s).map_err(|_| anyhow::anyhow!(format!("failed to parse card: {}", s)))?;
        anyhow::ensure!(
            s.is_empty(),
            format!("not all input was parsed, remaining: {}", s)
        );
        Ok(card)
    }
}

impl Card {
    fn matching(&self) -> Vec<usize> {
        self.winning_numbers
            .intersection(&self.my_numbers)
            .copied()
            .collect()
    }

    fn num_matching(&self) -> usize {
        self.winning_numbers.intersection(&self.my_numbers).count()
    }

    fn points(&self) -> usize {
        if self.matching().is_empty() {
            0
        } else {
            let exp = (self.matching().len() - 1) as u32;
            usize::pow(2, exp)
        }
    }
}

// Game is a collection of scratchcards
#[derive(Debug)]
struct Game {
    cards: Vec<Card>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in self.cards.iter() {
            writeln!(f, "{}", card)?;
        }
        Ok(())
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s.lines().map(str::parse).collect::<Result<Vec<_>>>()?;
        Ok(Game { cards })
    }
}

impl Game {
    fn matching(&self) -> Vec<Vec<usize>> {
        self.cards.iter().map(Card::matching).collect::<Vec<_>>()
    }

    fn points(&self) -> usize {
        self.cards.iter().map(Card::points).sum()
    }

    fn play(&mut self) -> usize {
        for card_idx in 0..self.cards.len() {
            let card @ &Card { id, copies, .. } = &self.cards[card_idx];
            let num_matching = card.num_matching();
            for next_card in self.cards.iter_mut().skip(id).take(num_matching) {
                next_card.copies += copies;
            }
        }
        tracing::debug!("cards playing the game:\n{}", self);
        self.cards.iter().map(|c| c.copies).sum()
    }
}
fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space1, parse_number)(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, (_, _, id, _, (winning_numbers, my_numbers))) = tuple((
        tag("Card"),
        space1,
        parse_number,
        char(':'),
        separated_pair(
            delimited(space1, parse_numbers, space0),
            char('|'),
            delimited(space1, parse_numbers, space0),
        ),
    ))(input)?;
    let card = Card {
        id,
        copies: 1,
        winning_numbers: winning_numbers.into_iter().collect(),
        my_numbers: my_numbers.into_iter().collect(),
    };
    Ok((input, card))
}

pub fn part1_and_part2() -> Result<()> {
    let input = include_str!("../../input/day04.txt");
    let mut game = input.parse::<Game>()?;
    tracing::debug!("games:\n{}", game);
    for (i, numbers) in game.matching().iter().enumerate() {
        tracing::debug!("Matching numbers in card {}: {:?}", i + 1, numbers);
    }

    let part1 = game.points();
    tracing::info!("[part1] Elf's scratchcards are worth {} points", part1);
    // assert_eq!(part1, 20829);

    let part2 = game.play();
    tracing::info!("[part2] Elf won a total of {} scratchcards", part2);
    assert_eq!(part2, 12648035);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() -> Result<()> {
        let input = include_str!("../../sample/day04.txt");
        let mut game = input.parse::<Game>()?;

        let part1 = game.points();
        assert_eq!(part1, 13);

        let part2 = game.play();
        assert_eq!(part2, 30);

        Ok(())
    }
}
