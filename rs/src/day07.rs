use std::{cmp::Ordering, str::FromStr};

use anyhow::Result;
use itertools::Itertools;
use nom::{
    character::complete::{alphanumeric1, digit1, space1},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
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
    J, // Jack / Joker
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
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<[usize; 13]> for HandType {
    fn from(counts: [usize; 13]) -> Self {
        if counts.iter().any(|&count| count == 5) {
            // All five cards have the same label
            // e.g., AAAAA
            HandType::FiveOfAKind
        } else if counts.iter().any(|&count| count == 4) {
            // Four cards have the same label and one card has a different label
            // e.g., AA8AA
            assert!(counts.iter().filter(|&&count| count == 4).count() == 1);
            HandType::FourOfAKind
        } else if counts.iter().any(|&count| count == 3) && counts.iter().any(|&count| count == 2) {
            // Three cards have the same label, and the remaining two cards share a different label
            // e.g., 23332
            HandType::FullHouse
        } else if counts.iter().any(|&count| count == 3) {
            // Three cards have the same label, and the remaining two cards are each different
            // from any other card in the hand
            // e.g., TTT98
            assert!(counts.iter().filter(|&&count| count == 1).count() == 2);
            HandType::ThreeOfAKind
        } else if counts.iter().filter(|&&count| count == 2).count() == 2 {
            // Two cards share one label, two other cards share a second label,
            // and the remaining card has a third label
            // e.g., 23432
            assert!(counts.iter().filter(|&&count| count == 1).count() == 1);
            HandType::TwoPairs
        } else if counts.iter().any(|&count| count == 2) {
            // Two cards share one label, and the other three cards have a different label
            // from the pair and each other
            // e.g., A23A4
            assert!(counts.iter().filter(|&&count| count == 1).count() == 3);
            HandType::OnePair
        } else {
            // All cards' labels are distinct
            // e.g., 23456
            assert!(counts.iter().filter(|&&count| count == 1).count() == 5);
            HandType::HighCard
        }
    }
}

impl<'a> From<&'a DefaultHand> for HandType {
    fn from(value: &'a DefaultHand) -> Self {
        let counts = value
            .0
             .0
            .iter()
            .fold([0usize; Card::NUM_CARDS], |mut counts, card| {
                counts[*card as usize] += 1;
                counts
            });
        counts.into()
    }
}

impl<'a> From<&'a JokerHand> for HandType {
    fn from(value: &'a JokerHand) -> Self {
        let mut counts = value
            .0
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

        counts.into()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq)]
struct DefaultHand(Hand);

impl Ord for DefaultHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let kind = HandType::from(self);
        let other_kind = HandType::from(other);
        kind.cmp(&other_kind).then(self.0.cmp(&other.0))
    }
}

impl PartialOrd for DefaultHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct JokerHand(Hand);

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let kind = HandType::from(self);
        let other_kind = HandType::from(other);
        kind.cmp(&other_kind).then({
            let hand = self.0;
            let other_hand = other.0;
            hand.0
                .iter()
                .zip(other_hand.0.iter())
                .map(|(card, other_card)| match (card, other_card) {
                    (Card::J, Card::J) => Ordering::Equal,
                    (Card::J, _) => Ordering::Less,
                    (_, Card::J) => Ordering::Greater,
                    _ => card.cmp(other_card),
                })
                .find(|&order| order != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        })
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    fn winnings(&self) -> usize {
        self.0
            .iter()
            .sorted_by(|game, other_game| {
                let hand = DefaultHand(game.hand);
                let other_hand = DefaultHand(other_game.hand);
                hand.cmp(&other_hand)
            })
            .enumerate()
            .map(|(i, game)| game.bid * (i + 1))
            .sum()
    }

    fn winnings_with_joker(&self) -> usize {
        self.0
            .iter()
            .sorted_by(|game, other_game| {
                let hand = JokerHand(game.hand);
                let other_hand = JokerHand(other_game.hand);
                hand.cmp(&other_hand)
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

    let part1 = games.winnings();
    tracing::info!("[part 1] total winnings: {}", part1);
    assert_eq!(part1, 250602641);

    let part2 = games.winnings_with_joker();
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

        let part1 = games.winnings();
        assert_eq!(part1, 6440);

        let part2 = games.winnings_with_joker();
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
