use std::{cmp::Ordering, str::FromStr};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    character::complete::{alphanumeric1, digit1, space1},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J, // Jack or Joker
    Q, // Queen
    K, // King
    A, // Ace
}

impl TryFrom<u8> for Card {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            b'2' => Ok(Card::Two),
            b'3' => Ok(Card::Three),
            b'4' => Ok(Card::Four),
            b'5' => Ok(Card::Five),
            b'6' => Ok(Card::Six),
            b'7' => Ok(Card::Seven),
            b'8' => Ok(Card::Eight),
            b'9' => Ok(Card::Nine),
            b'T' => Ok(Card::Ten),
            b'J' => Ok(Card::J),
            b'Q' => Ok(Card::Q),
            b'K' => Ok(Card::K),
            b'A' => Ok(Card::A),
            _ => Err(anyhow::anyhow!("Invalid card: {}", value)),
        }
    }
}

impl Card {
    const NUM_CARDS: usize = 13;

    fn j_as_jack_cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }

    fn j_as_joker_cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Card::J, Card::J) => Ordering::Equal,
            (Card::J, _) => Ordering::Less,
            (_, Card::J) => Ordering::Greater,
            _ => self.cmp(other),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    fn from_counts(counts: [usize; 13]) -> Self {
        if counts.iter().any(|&count| count == 5) {
            // All five cards have the same label
            // e.g., AAAAA
            HandKind::FiveOfAKind
        } else if counts.iter().any(|&count| count == 4) {
            // Four cards have the same label and one card has a different label
            // e.g., AA8AA
            assert!(counts.iter().filter(|&&count| count == 4).count() == 1);
            HandKind::FourOfAKind
        } else if counts.iter().any(|&count| count == 3) && counts.iter().any(|&count| count == 2) {
            // Three cards have the same label, and the remaining two cards share a different label
            // e.g., 23332
            HandKind::FullHouse
        } else if counts.iter().any(|&count| count == 3) {
            // Three cards have the same label, and the remaining two cards are each different
            // from any other card in the hand
            // e.g., TTT98
            assert!(counts.iter().filter(|&&count| count == 1).count() == 2);
            HandKind::ThreeOfAKind
        } else if counts.iter().filter(|&&count| count == 2).count() == 2 {
            // Two cards share one label, two other cards share a second label,
            // and the remaining card has a third label
            // e.g., 23432
            assert!(counts.iter().filter(|&&count| count == 1).count() == 1);
            HandKind::TwoPairs
        } else if counts.iter().any(|&count| count == 2) {
            // Two cards share one label, and the other three cards have a different label
            // from the pair and each other
            // e.g., A23A4
            assert!(counts.iter().filter(|&&count| count == 1).count() == 3);
            HandKind::OnePair
        } else {
            // All cards' labels are distinct
            // e.g., 23456
            assert!(counts.iter().filter(|&&count| count == 1).count() == 5);
            HandKind::HighCard
        }
    }

    fn from_with_j_as_jack(value: Hand) -> Self {
        let counts = value
            .0
            .iter()
            .fold([0usize; Card::NUM_CARDS], |mut counts, card| {
                counts[*card as usize] += 1;
                counts
            });
        Self::from_counts(counts)
    }

    fn from_with_j_as_joker(value: Hand) -> Self {
        let mut counts = value
            .0
            .iter()
            .fold([0usize; Card::NUM_CARDS], |mut counts, card| {
                counts[*card as usize] += 1;
                counts
            });
        let jokers_count = counts[Card::J as usize];
        // remove jokers from counts
        counts[Card::J as usize] = 0;

        // give jokers to the most frequent card
        if let Some(max_value) = counts.iter_mut().max() {
            *max_value += jokers_count;
        }

        Self::from_counts(counts)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Hand([Card; 5]);

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        anyhow::ensure!(s.len() == 5, "invalid hand: {}", s);
        let mut cards = [Card::Two; 5];
        for (i, byte) in s.bytes().enumerate() {
            cards[i] = Card::try_from(byte)?;
        }
        Ok(Hand(cards))
    }
}

#[derive(Debug)]
struct Game {
    hand: Hand,
    bid: usize,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, (s, bid)) =
            parse_game(s).map_err(|_| anyhow::anyhow!("failed to parse input: {}", s))?;
        let cards = s.parse::<Hand>()?;
        Ok(Game { hand: cards, bid })
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

impl Games {
    fn winnings_with_j_as_jack(&self) -> usize {
        self.0
            .iter()
            .sorted_by(|game, other_game| {
                let hand_kind = HandKind::from_with_j_as_jack(game.hand);
                let other_hand_kind = HandKind::from_with_j_as_jack(other_game.hand);
                match hand_kind.cmp(&other_hand_kind) {
                    Ordering::Equal => game
                        .hand
                        .0
                        .iter()
                        .zip(other_game.hand.0)
                        .map(|(card, other_card)| card.j_as_jack_cmp(&other_card))
                        .find(|&order| order != Ordering::Equal)
                        .unwrap_or(Ordering::Equal),
                    order => order,
                }
            })
            .enumerate()
            .map(|(i, game)| game.bid * (i + 1))
            .sum()
    }

    fn winnings_with_j_as_joker(&self) -> usize {
        self.0
            .iter()
            .sorted_by(|game, other_game| {
                let hand_kind = HandKind::from_with_j_as_joker(game.hand);
                let other_hand_kind = HandKind::from_with_j_as_joker(other_game.hand);
                match hand_kind.cmp(&other_hand_kind) {
                    Ordering::Equal => game
                        .hand
                        .0
                        .iter()
                        .zip(other_game.hand.0)
                        .map(|(card, other_card)| card.j_as_joker_cmp(&other_card))
                        .find(|&order| order != Ordering::Equal)
                        .unwrap_or(Ordering::Equal),
                    order => order,
                }
            })
            .enumerate()
            .map(|(i, game)| game.bid * (i + 1))
            .sum()
    }
}
fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_game(input: &str) -> IResult<&str, (&str, usize)> {
    let (input, (hand, _, bid)) = tuple((alphanumeric1, space1, parse_number))(input)?;
    Ok((input, (hand, bid)))
}

pub fn part1_and_part2() -> Result<()> {
    let input = include_str!("../../input/day07.txt");
    let games = input.parse::<Games>()?;

    let part1 = games.winnings_with_j_as_jack();
    tracing::info!("[part 1] total winnings: {}", part1);
    assert_eq!(part1, 250602641);

    let part2 = games.winnings_with_j_as_joker();
    tracing::info!("[part 2] total winnings: {}", part2);
    assert_eq!(part2, 251037509);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() -> Result<()> {
        let input = include_str!("../../sample/day07.txt");
        let games = input.parse::<Games>()?;

        let part1 = games.winnings_with_j_as_jack();
        assert_eq!(part1, 6440);

        let part2 = games.winnings_with_j_as_joker();
        assert_eq!(part2, 5905);
        Ok(())
    }

    #[test]
    fn test_parse_game() -> Result<()> {
        let input = "32T3K 765";
        let (input, (hand, bid)) = parse_game(input)?;
        assert_eq!(input, "");
        assert_eq!(hand, "32T3K");
        assert_eq!(bid, 765);

        Ok(())
    }
}
