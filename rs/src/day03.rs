use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
    ops::Add,
    str::FromStr,
};

use anyhow::Result;
use nom::{
    branch::alt,
    character::complete::{anychar, char, digit1},
    combinator::{map, map_res},
    multi::many1,
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(isize, isize);

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Pos(x, y) = self;
        write!(f, "({}, {})", x, y)
    }
}

impl<T> Add<T> for Pos
where
    T: Borrow<Pos>,
{
    type Output = Pos;

    fn add(self, other: T) -> Self::Output {
        let Pos(x1, y1) = self;
        let Pos(x2, y2) = other.borrow();
        Pos(x1 + x2, y1 + y2)
    }
}

impl<T> Add<T> for &Pos
where
    T: Borrow<Pos>,
{
    type Output = Pos;

    fn add(self, other: T) -> Self::Output {
        let Pos(x1, y1) = self;
        let Pos(x2, y2) = other.borrow();
        Pos(x1 + x2, y1 + y2)
    }
}

impl Pos {
    const NEIGHBORS: [Pos; 8] = [
        Pos(0, 1),
        Pos(1, 1),
        Pos(1, 0),
        Pos(1, -1),
        Pos(0, -1),
        Pos(-1, -1),
        Pos(-1, 0),
        Pos(-1, 1),
    ];

    fn new(x: usize, y: usize) -> Self {
        Pos(x as isize, y as isize)
    }

    // Neighbors of self along x-axis, y-axis and diagonals
    fn neighbors(&self) -> Vec<Pos> {
        Pos::NEIGHBORS.iter().map(|p| self + p).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cell {
    Number { num: usize, len: usize },
    Dot,
    Symbol(char),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Number { num, .. } => write!(f, "{}", num),
            Cell::Dot => write!(f, "."),
            Cell::Symbol(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug)]
struct Engine {
    grid: Vec<Vec<Cell>>,
    pos_2_cells: HashMap<Pos, Cell>,
}

impl FromStr for Engine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| {
                parse_cells(line)
                    .map_err(|_| anyhow::anyhow!("parse error on line: {}", line))
                    .map(|(_, cells)| cells)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Engine::new(grid))
    }
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cells in self.grid.iter() {
            for cell in cells.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Engine {
    fn new(grid: Vec<Vec<Cell>>) -> Self {
        let mut pos_2_cells = HashMap::<Pos, Cell>::new();

        for (row, cells) in grid.iter().enumerate() {
            let mut col = 0;
            for cell in cells.iter() {
                match cell {
                    &Cell::Number { len, .. } => {
                        for i in 0..len {
                            let pos: Pos = Pos::new(row, col + i);
                            pos_2_cells.insert(pos, cell.clone());
                        }
                        col += len;
                    }
                    &Cell::Dot | &Cell::Symbol(_) => {
                        let pos: Pos = Pos::new(row, col);
                        pos_2_cells.insert(pos, cell.clone());
                        col += 1;
                    }
                }
            }
        }
        Engine { grid, pos_2_cells }
    }

    fn parts(&self) -> Vec<usize> {
        let mut part_numbers = vec![];
        for (row, cells) in self.grid.iter().enumerate() {
            let mut col = 0;
            for cell in cells.iter() {
                match cell {
                    &Cell::Number { num, len } => {
                        let pos: Pos = Pos::new(row, col);
                        let neighbors = (0..len)
                            .map(|i| Pos::new(0, i))
                            .map(|p| pos + p)
                            .flat_map(|p| p.neighbors())
                            .collect::<HashSet<_>>();
                        let is_part = neighbors
                            .iter()
                            .any(|p| matches!(self.get_cell(*p), Some(Cell::Symbol(_))));
                        if is_part {
                            part_numbers.push(num);
                        }
                        col += len;
                    }
                    &Cell::Dot | &Cell::Symbol(_) => {
                        col += 1;
                    }
                }
            }
        }
        part_numbers
    }

    fn gears(&self) -> Vec<Vec<usize>> {
        let mut gears = vec![];
        for (row, cells) in self.grid.iter().enumerate() {
            let mut col = 0;
            for cell in cells.iter() {
                match cell {
                    Cell::Number { len, .. } => {
                        col += len;
                    }
                    Cell::Dot => {
                        col += 1;
                    }
                    Cell::Symbol('*') => {
                        let pos = Pos::new(row, col);
                        let neighbors = pos.neighbors();
                        let neighbor_numbers = neighbors
                            .iter()
                            .filter_map(|&p| {
                                self.get_cell(p)
                                    .filter(|&cell| matches!(cell, Cell::Number { .. }))
                                    .cloned()
                            })
                            .collect::<HashSet<_>>();
                        if neighbor_numbers.len() == 2 {
                            let nums = neighbor_numbers
                                .iter()
                                .filter_map(|cell| match cell {
                                    Cell::Number { num, .. } => Some(*num),
                                    _ => None,
                                })
                                .collect::<Vec<_>>();
                            gears.push(nums);
                        }
                        col += 1;
                    }
                    Cell::Symbol(_) => {
                        col += 1;
                    }
                }
            }
        }
        gears
    }

    fn sum_of_parts(&self) -> usize {
        self.parts().iter().sum()
    }

    fn get_cell(&self, pos: Pos) -> Option<&Cell> {
        self.pos_2_cells.get(&pos)
    }
}

fn parse_number(input: &str) -> IResult<&str, (usize, usize)> {
    let len = input.len();
    let (input, num) = map_res(digit1, |d: &str| d.parse::<usize>())(input)?;
    Ok((input, (num, len - input.len())))
}

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    alt((
        map(parse_number, |(num, len)| Cell::Number { num, len }),
        map(char('.'), |_| Cell::Dot),
        map(anychar, Cell::Symbol),
    ))(input)
}

fn parse_cells(input: &str) -> IResult<&str, Vec<Cell>> {
    many1(parse_cell)(input)
}

pub fn part1_and_part2() -> Result<()> {
    let input = include_str!("../../input/day03.txt");
    let engine = input.parse::<Engine>()?;
    let parts = engine.parts();
    tracing::debug!("engine:\n{}", engine);
    tracing::debug!("parts: {:?}", parts);
    let part1 = engine.sum_of_parts();
    tracing::info!("[part 1] sum of all part numbers: {}", part1);
    assert_eq!(part1, 557705);

    let gears = engine.gears();
    tracing::debug!("gears: {:?}", gears);
    let gear_ratios = gears
        .into_iter()
        .map(|nums| nums.into_iter().product::<usize>())
        .collect::<Vec<_>>();
    tracing::debug!("gears: {:?}", gear_ratios);
    let part2 = gear_ratios.iter().sum::<usize>();
    tracing::info!("[part 2] sum of all the gear ratios: {}", part2);
    assert_eq!(part2, 84266818);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() -> Result<()> {
        let input = include_str!("../../sample/day03.txt");
        let engine = input.parse::<Engine>()?;
        let part1 = engine.sum_of_parts();
        assert_eq!(part1, 4361);

        let gears = engine.gears();
        let gear_ratios = gears
            .into_iter()
            .map(|nums| nums.into_iter().product::<usize>())
            .collect::<Vec<_>>();
        let part2 = gear_ratios.iter().sum::<usize>();
        assert_eq!(part2, 467835);
        Ok(())
    }
}
