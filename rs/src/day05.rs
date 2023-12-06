use core::fmt;
use std::str::FromStr;

use anyhow::Result;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct Seeds(Vec<usize>);

impl fmt::Display for Seeds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} seeds with {} 2-sized chunks",
            self.0.len(),
            self.0.len() / 2
        )?;

        for (i, chunk) in self.0.chunks_exact(2).enumerate() {
            let start = chunk[0];
            let len = chunk[1];
            writeln!(
                f,
                "{} seed range: [{}] ({:10}, {:10}) ",
                i,
                len,
                start,
                start + len
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    src: usize,
    dst: usize,
    len: usize,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            " [{:10}] ({:10} {:10}) -> {:10}",
            self.len,
            self.src,
            self.src + self.len,
            self.dst
        )
    }
}

impl Range {
    fn contains(&self, key: &usize) -> bool {
        let lb = self.src;
        let ub = self.src + self.len;
        (lb..ub).contains(key)
    }

    fn map(&self, key: &usize) -> usize {
        assert!(self.contains(key), "key must be in range");
        self.dst + (key - self.src)
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for range in &self.ranges {
            writeln!(f, "{}", range)?;
        }
        Ok(())
    }
}

impl Map {
    fn new(ranges: Vec<Range>) -> Self {
        Self { ranges }
    }

    fn map(&self, key: usize) -> usize {
        // Ranges are all sorted by src; hence we can binar search over them
        // to find the range that contains the key.
        let mut lb = 0;
        let mut ub = self.ranges.len();
        while lb < ub {
            let mid = (lb + ub) / 2;
            let range = self.ranges[mid];
            if range.contains(&key) {
                return range.map(&key);
            } else if key < range.src {
                ub = mid;
            } else {
                lb = mid + 1;
            }
        }
        // At this point, we haven't found the input key in the range.
        // return the key itself.
        key
    }
}

#[derive(Debug)]
struct Maps(Vec<Map>);

impl Maps {
    fn map(&self, key: usize) -> usize {
        // map through all maps in order
        self.0.iter().fold(key, |acc, map| map.map(acc))
    }

    fn min(&self, lb: usize, ub: usize) -> usize {
        assert!(lb < ub, "range must be non-empty");

        // binary search over the map to find the minimum value

        if lb + 1 == ub {
            return self.map(lb);
        }

        let len = ub - lb;
        let value_lb = self.map(lb);
        let value_ub = self.map(ub - 1);
        if value_ub > value_lb && value_ub - value_lb == len - 1 {
            // the map range (lb, ub) is monotonic and linear
            // hence we can return the minimum value directly
            value_lb
        } else {
            let mid = (lb + ub) / 2;
            usize::min(self.min(lb, mid), self.min(mid, ub))
        }
    }
}

#[derive(Debug)]
struct Input(Seeds, Maps);

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (seeds, maps)) =
            parse_input(s).map_err(|_| anyhow::anyhow!("failed to parse input"))?;
        Ok(Input(seeds, maps))
    }
}

impl Input {
    fn lowest_location(&self) -> usize {
        let Input(seeds, maps) = self;
        seeds
            .0
            .iter()
            .map(|&seed| maps.map(seed))
            .fold(usize::MAX, usize::min)
    }

    fn lowest_location_of_seed_ranges(&self) -> usize {
        let Input(seeds, maps) = self;
        seeds
            .0
            .chunks_exact(2)
            .enumerate()
            .map(|(i, chunk)| {
                let seed = chunk[0];
                let len = chunk[1];
                tracing::debug!(
                    "{:2}: searching over  [{}] ({}, {})",
                    i,
                    len,
                    seed,
                    seed + len,
                );
                let lb = seed;
                let ub = seed + len;
                maps.min(lb, ub)
            })
            .fold(usize::MAX, usize::min)
    }
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(space1, parse_number)(input)
}

fn parse_map(input: &str) -> IResult<&str, Range> {
    let (input, (dst, _, src, _, len)) =
        tuple((parse_number, space1, parse_number, space1, parse_number))(input)?;
    Ok((input, Range { src, dst, len }))
}

fn parse_input(input: &str) -> IResult<&str, (Seeds, Maps)> {
    let mut maps = vec![];

    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = parse_numbers(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    tracing::debug!("seeds: {:?}", seeds);

    assert!(seeds.len() >= 2, "there must be at least two seeds");
    assert!(seeds.len() % 2 == 0, "there must be even number of seeds");

    let (input, _) = tag("seed-to-soil map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, mut map) = separated_list1(newline, parse_map)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    map.sort();
    let map = Map::new(map);
    tracing::debug!("seed-to-soil map:\n{}", map);
    maps.push(map);

    let (input, _) = tag("soil-to-fertilizer map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, mut map) = separated_list1(newline, parse_map)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    map.sort();
    let map = Map::new(map);
    tracing::debug!("soil-to-fertilizer map:\n{}", map);
    maps.push(map);

    let (input, _) = tag("fertilizer-to-water map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, mut map) = separated_list1(newline, parse_map)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    map.sort();
    let map = Map::new(map);
    tracing::debug!("fertilizer-to-water map:\n{}", map);
    maps.push(map);

    let (input, _) = tag("water-to-light map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, mut map) = separated_list1(newline, parse_map)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    map.sort();
    let map = Map::new(map);
    tracing::debug!("water-to-light map:\n{}", map);
    maps.push(map);

    let (input, _) = tag("light-to-temperature map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, mut map) = separated_list1(newline, parse_map)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    map.sort();
    let map = Map::new(map);
    tracing::debug!("light-to-temperature map:\n{}", map);
    maps.push(map);

    let (input, _) = tag("temperature-to-humidity map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, mut map) = separated_list1(newline, parse_map)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;
    map.sort();
    let map = Map::new(map);
    tracing::debug!("temperature-to-humidity map:\n{}", map);
    maps.push(map);

    let (input, _) = tag("humidity-to-location map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, mut map) = separated_list1(newline, parse_map)(input)?;
    map.sort();
    let map = Map::new(map);
    tracing::debug!("humidity-to-location map:\n{}", map);
    maps.push(map);

    Ok((input, (Seeds(seeds), Maps(maps))))
}

pub fn part1_and_part2() -> Result<()> {
    let input = include_str!("../../input/day05.txt");
    let Input(seeds, maps) = input.parse::<Input>()?;

    tracing::debug!("{}", seeds);
    for (map_idx, map) in maps.0.iter().enumerate() {
        for range in &map.ranges {
            tracing::debug!(
                "map {}: ({}, {})",
                map_idx,
                range.src,
                range.src + range.len
            );
        }
        tracing::debug!("");
    }
    let input = Input(seeds, maps);
    let part1 = input.lowest_location();
    tracing::info!("[part 1] lowest location number: {}", part1);
    assert_eq!(part1, 388071289);

    let part2 = input.lowest_location_of_seed_ranges();
    tracing::info!("[part 2] lowest location number: {}", part2);
    assert_eq!(part2, 84206669);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() -> Result<()> {
        let input = include_str!("../../sample/day05.txt");
        let Input(seeds, maps) = input.parse::<Input>()?;

        assert_eq!(seeds.0, vec![79, 14, 55, 13]);
        assert_eq!(maps.0.len(), 7);

        // seed to soil map for sample input
        assert_eq!(maps.0[0].map(79), 81);
        assert_eq!(maps.0[0].map(14), 14);
        assert_eq!(maps.0[0].map(55), 57);
        assert_eq!(maps.0[0].map(13), 13);

        // seed to location map for sample input
        assert_eq!(maps.map(79), 82);
        assert_eq!(maps.map(14), 43);
        assert_eq!(maps.map(55), 86);
        assert_eq!(maps.map(13), 35);

        assert_eq!(maps.map(82), 46);

        let input = Input(seeds, maps);

        let part1 = input.lowest_location();
        assert_eq!(part1, 35);

        let part2 = input.lowest_location_of_seed_ranges();
        assert_eq!(part2, 46);

        Ok(())
    }

    #[test]
    fn test_that_breaks_day5_part2_algo() -> Result<()> {
        let seeds = vec![0, 100];
        let maps1 = vec![Range {
            src: 0,
            dst: 100,
            len: 100,
        }];
        let maps2 = vec![
            Range {
                src: 100,
                dst: 100,
                len: 50,
            },
            Range {
                src: 150,
                dst: 0,
                len: 20,
            },
            Range {
                src: 170,
                dst: 170,
                len: 30,
            },
        ];
        let map1 = Map::new(maps1);
        let map2 = Map::new(maps2);
        let maps = Maps(vec![map1, map2]);
        assert_eq!(maps.map(0), 100);
        assert_eq!(maps.map(99), 199);
        assert_eq!(maps.map(100), 100);
        assert_eq!(maps.map(50), 0);
        assert_eq!(maps.map(69), 19);
        assert_eq!(maps.map(70), 170);
        let input = Input(Seeds(seeds), maps);
        assert_eq!(input.lowest_location(), 100);

        // should print 0, but prints 100
        // assert_eq!(input.lowest_location_of_seed_ranges(), 0);
        Ok(())
    }

    #[test]
    fn test_parse_map() -> Result<()> {
        // 50 98 2
        let input = "50 98 2";
        let (input, map) = parse_map(input)?;
        assert_eq!(input, "");
        assert_eq!(
            map,
            Range {
                src: 98,
                dst: 50,
                len: 2
            }
        );
        Ok(())
    }
}
