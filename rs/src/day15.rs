use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit0},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Clone)]
enum Op {
    Remove,
    Add(usize),
}

fn parse_digit(input: &str) -> IResult<&str, usize> {
    let (input, digit) = map_res(digit0, |digit: &str| digit.parse::<usize>())(input)?;
    Ok((input, digit))
}

fn parse_remove_op(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("-")(input)?;
    Ok((input, Op::Remove))
}

fn parse_add_op(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("=")(input)?;
    let (input, num) = parse_digit(input)?;
    Ok((input, Op::Add(num)))
}

fn parse_op(input: &str) -> IResult<&str, Op> {
    let (input, op) = alt((parse_remove_op, parse_add_op))(input)?;
    Ok((input, op))
}

fn parse_step(s: &str) -> IResult<&str, Step> {
    let (input, (label, op)) = tuple((alpha1, parse_op))(s)?;
    Ok((
        input,
        Step {
            inner: s,
            label,
            op,
        },
    ))
}

#[derive(Debug, Clone)]
struct Step<'a> {
    inner: &'a str,
    label: &'a str,
    op: Op,
}

impl<'a> TryFrom<&'a str> for Step<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self> {
        let (_, step) =
            parse_step(s).map_err(|_| anyhow::anyhow!("failed to parse step: {}", s))?;
        Ok(step)
    }
}

impl<'a> Step<'a> {
    fn hash(bytes: &[u8]) -> usize {
        let mut hash = 0usize;
        for &b in bytes {
            hash = hash.wrapping_add(b as usize);
            hash = hash.wrapping_mul(17);
            hash = hash.wrapping_rem(256);
        }
        hash
    }

    fn hash_step(&self) -> usize {
        Step::hash(self.inner.as_bytes())
    }

    fn hash_label(&self) -> usize {
        Step::hash(self.label.as_bytes())
    }
}

#[derive(Debug)]
struct Steps<'a>(Vec<Step<'a>>);

impl<'a> TryFrom<&'a str> for Steps<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self> {
        let steps = s
            .split(',')
            .map(Step::try_from)
            .collect::<Result<Vec<_>>>()?;
        Ok(Steps(steps))
    }
}

impl<'a> Steps<'a> {
    fn sum_of_hashes(&self) -> usize {
        self.0.iter().map(|s| s.hash_step()).sum()
    }

    fn run(&self) -> usize {
        let mut boxes: Vec<Vec<Step<'_>>> = vec![vec![]; 256];
        for step in &self.0 {
            match step.op {
                Op::Remove => {
                    let hash = step.hash_label();
                    if let Some(step_idx) = boxes[hash].iter().position(|s| s.label == step.label) {
                        boxes[hash].remove(step_idx);
                    }
                }
                Op::Add(_) => {
                    let hash = step.hash_label();
                    if let Some(step_idx) = boxes[hash].iter().position(|s| s.label == step.label) {
                        boxes[hash][step_idx] = step.clone();
                    } else {
                        boxes[hash].push(step.clone());
                    }
                }
            }
        }

        let mut power = 0;
        for (bx_idx, bx) in boxes.iter().enumerate() {
            for (step_idx, step) in bx.iter().enumerate() {
                let focal_length = match step.op {
                    Op::Remove => unreachable!("shouldn't be any remove ops in boxes"),
                    Op::Add(num) => num,
                };
                power += (bx_idx + 1) * (step_idx + 1) * focal_length;
            }
        }
        power
    }
}

pub fn part1() -> Result<()> {
    let input = include_str!("../../input/day15.txt");
    let steps = Steps::try_from(input)?;
    let part1 = steps.sum_of_hashes();
    tracing::info!("[part 1] sum of hashes: {}", part1);
    Ok(())
}

pub fn part2() -> Result<()> {
    let input = include_str!("../../input/day15.txt");
    let steps = Steps::try_from(input)?;
    let part2 = steps.run();
    tracing::info!("[part 2] total focusing power: {}", part2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() -> Result<()> {
        let step = Step::try_from("rn=1")?;
        assert_eq!(step.hash_step(), 30);
        assert_eq!(step.hash_label(), 0);

        let step = Step::try_from("cm-")?;
        assert_eq!(step.hash_step(), 253);
        assert_eq!(step.hash_label(), 0);

        let step = Step::try_from("qp=3")?;
        assert_eq!(step.hash_step(), 97);
        assert_eq!(step.hash_label(), 1);
        Ok(())
    }

    #[test]
    fn test_with_sample() -> Result<()> {
        let input = include_str!("../../sample/day15.txt");
        let steps = Steps::try_from(input)?;
        let part1 = steps.sum_of_hashes();
        assert_eq!(part1, 1320);

        let part2 = steps.run();
        assert_eq!(part2, 145);
        Ok(())
    }
}
