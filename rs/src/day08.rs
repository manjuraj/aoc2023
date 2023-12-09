use std::{collections::HashMap, fmt, str::FromStr};

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::{complete::newline, is_alphanumeric},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    IResult,
};

pub fn part1_and_part2() -> Result<()> {
    let input = include_str!("../../input/day08.txt").parse::<Input>()?;
    let part1 = input.steps();
    tracing::info!("[part 1]: # steps to reach ZZZ: {}", part1);

    let part2 = input.multi_steps();
    tracing::info!(
        "[part 2]: # steps to reach all labels ending in Z: {}",
        part2
    );
    Ok(())
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction(Vec<Direction>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Label([u8; 3]);

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl TryFrom<&[u8]> for Label {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        anyhow::ensure!(
            value.len() == 3,
            "Label '{:?}' must be 3 characters long",
            value
        );
        let mut name = [0; 3];
        name.copy_from_slice(value);
        Ok(Label(name))
    }
}

impl Label {
    const START: Label = Label([b'A', b'A', b'A']);
    const END: Label = Label([b'Z', b'Z', b'Z']);
}

#[derive(Debug, Clone)]
struct Node {
    name: Label,
    left: Label,
    right: Label,
}

#[derive(Debug)]
struct Input {
    instruction: Instruction,
    nodes: Vec<Node>,
    labels: HashMap<Label, Node>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, input) =
            parse_input(s.as_bytes()).map_err(|_| anyhow::anyhow!("failed to parse input"))?;
        Ok(input)
    }
}

impl Input {
    fn steps(&self) -> usize {
        let mut steps = 0usize;
        let mut label = Label::START;
        for direction in self.instruction.0.iter().cycle() {
            let node = self.labels.get(&label).unwrap();
            label = match direction {
                Direction::Left => node.left,
                Direction::Right => node.right,
            };
            steps += 1;
            if label == Label::END {
                return steps;
            }
        }
        unreachable!()
    }

    fn multi_steps(&self) -> usize {
        // starting points are all labels that end with 'A'
        let starting_labels = self
            .nodes
            .iter()
            .filter(|node| node.name.0[2] == b'A')
            .cloned()
            .collect::<Vec<_>>();

        let steps = starting_labels
            .iter()
            .map(|starting_node| {
                let mut steps = 0usize;
                let mut label = starting_node.name;
                for direction in self.instruction.0.iter().cycle() {
                    let node = self.labels.get(&label).unwrap();
                    label = match direction {
                        Direction::Left => node.left,
                        Direction::Right => node.right,
                    };
                    steps += 1;
                    if label.0[2] == b'Z' {
                        break;
                    }
                }
                steps
            })
            .collect::<Vec<_>>();

        lcm_of_set(&steps).unwrap().get()
    }
}

use std::num::NonZeroUsize;

// Function to calculate the greatest common divisor (GCD)
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// Function to calculate the least common multiple (LCM) of two numbers
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

// Function to calculate the LCM of a set of numbers
fn lcm_of_set(numbers: &[usize]) -> Option<NonZeroUsize> {
    numbers
        .iter()
        .cloned()
        .reduce(lcm)
        .and_then(NonZeroUsize::new)
}

fn parse_label(input: &[u8]) -> IResult<&[u8], Label> {
    map_res(take_while_m_n(3, 3, is_alphanumeric), Label::try_from)(input)
}

fn parse_node(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, name) = parse_label(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, left) = parse_label(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, right) = parse_label(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Node { name, left, right }))
}

fn parse_direction(input: &[u8]) -> IResult<&[u8], Direction> {
    alt((
        map(tag("L"), |_| Direction::Left),
        map(tag("R"), |_| Direction::Right),
    ))(input)
}

fn parse_instruction(input: &[u8]) -> IResult<&[u8], Instruction> {
    let (input, directions) = many1(parse_direction)(input)?;
    Ok((input, Instruction(directions)))
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Input> {
    let (input, instruction) = parse_instruction(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, nodes) = separated_list1(newline, parse_node)(input)?;
    let labels = nodes
        .iter()
        .map(|node| (node.name, node.clone()))
        .collect::<HashMap<_, _>>();
    Ok((
        input,
        Input {
            instruction,
            nodes,
            labels,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() -> Result<()> {
        let input = include_str!("../../sample/day08.txt");
        let input = input.parse::<Input>()?;
        let part1 = input.steps();
        assert_eq!(part1, 2);

        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let input = input.parse::<Input>()?;
        let part1 = input.steps();
        assert_eq!(part1, 6);

        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let input = input.parse::<Input>()?;
        let part2 = input.multi_steps();
        assert_eq!(part2, 6);

        Ok(())
    }
}
