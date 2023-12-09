use std::str::FromStr;

use anyhow::Result;
use nom::{
    character::complete::{char, digit1, newline, space1},
    combinator::{map_res, recognize},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

#[derive(Debug)]
struct History(Vec<isize>);

impl History {
    fn next_value(&self) -> isize {
        let mut placeholders = vec![];
        let mut deltas = self.0.clone();

        loop {
            // push the last delta
            tracing::debug!("deltas: {:?}", deltas);
            placeholders.push(deltas[deltas.len() - 1]);

            // we're done if all deltas are 0
            if deltas.iter().all(|&d| d == 0) {
                break;
            }

            // otherwise, compute the next deltas
            deltas = deltas.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        }

        placeholders.reverse();
        tracing::debug!("placeholders: {:?}", placeholders);

        placeholders.into_iter().fold(0, |delta, curr| {
            let next_value = curr + delta;
            tracing::debug!("next value: {}", curr + delta);
            next_value
        })
    }
}

#[derive(Debug)]
struct Histories(Vec<History>);

impl FromStr for Histories {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, histories) =
            parse_histories(s).map_err(|_| anyhow::anyhow!("failed to parse input"))?;
        Ok(histories)
    }
}

impl Histories {
    fn next_values(&self) -> Vec<isize> {
        self.0.iter().map(|h| h.next_value()).collect::<Vec<_>>()
    }

    fn sum(&self) -> isize {
        self.next_values().iter().sum()
    }

    fn reverse_sum(&self) -> isize {
        let histories = self
            .0
            .iter()
            .map(|h| {
                let mut history = h.0.clone();
                history.reverse();
                History(history)
            })
            .collect::<Vec<_>>();
        let histories = Histories(histories);
        histories.next_values().iter().sum()
    }
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    let parse_negative = preceded(char('-'), digit1);
    let parse_number = recognize(parse_negative.or(digit1));

    map_res(parse_number, |s: &str| s.parse::<isize>())(input)
}

fn parse_history(input: &str) -> IResult<&str, History> {
    let (input, history) = separated_list1(space1, parse_isize)(input)?;
    Ok((input, History(history)))
}

fn parse_histories(input: &str) -> IResult<&str, Histories> {
    let (input, histories) = separated_list1(newline, parse_history)(input)?;
    Ok((input, Histories(histories)))
}

pub fn part1_and_part2() -> Result<()> {
    let input = include_str!("../../input/day09.txt");
    let histories = input.parse::<Histories>()?;

    let part1 = histories.sum();
    tracing::info!("[part 1]: sum of extrapolated values: {}", part1);

    let part2 = histories.reverse_sum();
    tracing::info!("[part 2]: sum of extrapolated values: {}", part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample_day09() -> Result<()> {
        let input = include_str!("../../sample/day09.txt");
        let histories = input.parse::<Histories>()?;
        let next_values = histories
            .0
            .iter()
            .map(|h| h.next_value())
            .collect::<Vec<_>>();
        assert_eq!(next_values, vec![18, 28, 68]);

        let part1 = histories.sum();
        assert_eq!(part1, 114);

        let part2 = histories.reverse_sum();
        assert_eq!(part2, 2);
        Ok(())
    }
}
