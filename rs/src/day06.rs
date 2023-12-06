use core::fmt;
use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "time: {}, distance: {}", self.time, self.distance)
    }
}

impl Race {
    fn distance(&self, hold_time: usize) -> usize {
        assert!(hold_time <= self.time);
        let remaining_time = self.time - hold_time;
        let speed = hold_time;
        remaining_time * speed
    }

    fn winning_bets(&self) -> Vec<(usize, usize)> {
        (0..=self.time)
            .map(|hold_time| {
                let distance = self.distance(hold_time);
                (hold_time, distance)
            })
            .skip_while(|&(_, distance)| distance <= self.distance)
            .take_while(|&(_, distance)| distance > self.distance)
            .collect::<Vec<_>>()
    }

    fn num_winning_bets(&self) -> usize {
        (0..=self.time)
            .map(|hold_time| {
                let distance = self.distance(hold_time);
                (hold_time, distance)
            })
            .skip_while(|&(_, distance)| distance <= self.distance)
            .take_while(|&(_, distance)| distance > self.distance)
            .count()
    }
}

#[derive(Debug)]
struct Races(Vec<Race>);

impl fmt::Display for Races {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, race) in self.0.iter().enumerate() {
            write!(f, "{} {}", i, race)?;
        }
        Ok(())
    }
}

impl FromStr for Races {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        parse_races(s)
            .map_err(|_| anyhow::anyhow!("failed to parse input"))
            .map(|(_, races)| races)
    }
}

impl Races {
    fn num_winning_bets(&self) -> usize {
        self.0
            .iter()
            .map(Race::num_winning_bets)
            .filter(|&len| len > 0)
            .product()
    }

    fn unkerned(&self) -> Race {
        let time = self
            .0
            .iter()
            .map(|race| race.time)
            .join("")
            .parse::<usize>()
            .unwrap();
        let distance = self
            .0
            .iter()
            .map(|race| race.distance)
            .join("")
            .parse::<usize>()
            .unwrap();
        Race { time, distance }
    }
}

pub fn part1_and_part2() -> Result<()> {
    let input = include_str!("../../input/day06.txt");
    let races = input.parse::<Races>()?;

    for (i, race) in races.0.iter().enumerate() {
        tracing::debug!("winning bet of race: {}", i);
        for (hold_time, distance) in race.winning_bets() {
            tracing::debug!("[{}] hold time: {}, distance: {}", i, hold_time, distance);
        }
    }
    let part1 = races.num_winning_bets();
    tracing::info!(
        "[part 1]: product of number of ways to beat the record in each race: {}",
        part1
    );
    assert_eq!(part1, 293046);

    let race = races.unkerned();
    let part2 = race.num_winning_bets();
    tracing::info!("[part 2]: number of ways to beat the record: {}", part2);
    assert_eq!(part2, 35150181);

    Ok(())
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space1, parse_number)(input)
}

fn parse_races(input: &str) -> IResult<&str, Races> {
    let (input, (_, _, times, _, _, _, distances)) = tuple((
        tag("Time:"),
        space1,
        parse_numbers,
        newline,
        tag("Distance:"),
        space1,
        parse_numbers,
    ))(input)?;
    assert_eq!(times.len(), distances.len());
    let races = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect::<Vec<_>>();
    Ok((input, Races(races)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() -> Result<()> {
        let input = include_str!("../../sample/day06.txt");
        let races = input.parse::<Races>()?;
        let part1 = races.num_winning_bets();
        assert_eq!(part1, 288);

        let race = races.unkerned();
        let part2 = race.num_winning_bets();
        assert_eq!(part2, 71503);
        Ok(())
    }
}
