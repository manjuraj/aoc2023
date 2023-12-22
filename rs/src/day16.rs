use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Entry {
    Empty,              // .
    RightMirror,        // /
    LeftMirror,         // \
    VerticalSplitter,   // |
    HorizontalSplitter, // -
}

impl TryFrom<u8> for Entry {
    type Error = anyhow::Error;

    fn try_from(b: u8) -> Result<Self> {
        match b {
            b'.' => Ok(Entry::Empty),
            b'/' => Ok(Entry::RightMirror),
            b'\\' => Ok(Entry::LeftMirror),
            b'|' => Ok(Entry::VerticalSplitter),
            b'-' => Ok(Entry::HorizontalSplitter),
            _ => Err(anyhow::anyhow!("Invalid entry: {}", b as char)),
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entry::Empty => write!(f, "."),
            Entry::RightMirror => write!(f, "/"),
            Entry::LeftMirror => write!(f, "\\"),
            Entry::VerticalSplitter => write!(f, "|"),
            Entry::HorizontalSplitter => write!(f, "-"),
        }
    }
}

// Grid is a 2D array of Entry in *row-major* order.
#[derive(Debug)]
struct Grid {
    entries: Vec<Vec<Entry>>,
    rows: usize,
    cols: usize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let entries = s
            .lines()
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .map(|&b| Entry::try_from(b))
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<Vec<_>>>>()?;
        let rows = entries.len();
        let cols = entries[0].len();
        Ok(Grid {
            entries,
            rows,
            cols,
        })
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} x {}", self.rows, self.cols)?;
        for row in &self.entries {
            for entry in row {
                write!(f, "{}", entry)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Traverse<'a> {
    grid: &'a Grid,
    visited: HashMap<(usize, usize), HashSet<Direction>>,
}

impl fmt::Display for Traverse<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.grid.rows {
            for col in 0..self.grid.cols {
                if let Some(entry) = self.visited.get(&(row, col)) {
                    // if entry.contains(&Direction::Up) {
                    //     write!(f, "↑")?;
                    // }
                    // if entry.contains(&Direction::Down) {
                    //     write!(f, "↓")?;
                    // }
                    // if entry.contains(&Direction::Left) {
                    //     write!(f, "←")?;
                    // }
                    // if entry.contains(&Direction::Right) {
                    //     write!(f, "→")?;
                    // }
                    if entry.is_empty() {
                        write!(f, "·")?;
                    } else {
                        write!(f, "#")?;
                    }
                } else {
                    write!(f, "·")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<'a> Traverse<'a> {
    fn new(grid: &'a Grid) -> Self {
        Self {
            grid,
            visited: HashMap::new(),
        }
    }

    fn traverse(&mut self, row: isize, col: isize, dir: Direction) {
        // base case
        if row < 0 || col < 0 || row >= self.grid.rows as isize || col >= self.grid.cols as isize {
            tracing::debug!("done: row={}, col={}, dir={:?}", row, col, dir);
            return;
        }

        // recursive case
        let row_usize = row as usize;
        let col_usize = col as usize;
        // mark (row, col) as visited, and increment the visit count
        let cached_entry = self.visited.entry((row_usize, col_usize)).or_default();
        if cached_entry.contains(&dir) {
            tracing::debug!("already visited: row={}, col={}, dir={:?}", row, col, dir);
            return;
        }
        cached_entry.insert(dir);

        let entry = &self.grid.entries[row_usize][col_usize];
        match (dir, entry) {
            // up
            (Direction::Up, Entry::Empty) => {
                self.traverse(row - 1, col, Direction::Up);
            }
            (Direction::Up, Entry::VerticalSplitter) => {
                self.traverse(row - 1, col, Direction::Up);
            }
            (Direction::Up, Entry::HorizontalSplitter) => {
                self.traverse(row, col - 1, Direction::Left);
                self.traverse(row, col + 1, Direction::Right);
            }
            (Direction::Up, Entry::LeftMirror) => {
                self.traverse(row, col - 1, Direction::Left);
            }
            (Direction::Up, Entry::RightMirror) => {
                self.traverse(row, col + 1, Direction::Right);
            }

            // right
            (Direction::Right, Entry::Empty) => {
                self.traverse(row, col + 1, Direction::Right);
            }
            (Direction::Right, Entry::VerticalSplitter) => {
                self.traverse(row - 1, col, Direction::Up);
                self.traverse(row + 1, col, Direction::Down);
            }
            (Direction::Right, Entry::HorizontalSplitter) => {
                self.traverse(row, col + 1, Direction::Right);
            }
            (Direction::Right, Entry::LeftMirror) => {
                self.traverse(row + 1, col, Direction::Down);
            }
            (Direction::Right, Entry::RightMirror) => {
                self.traverse(row - 1, col, Direction::Up);
            }

            // down
            (Direction::Down, Entry::Empty) => {
                self.traverse(row + 1, col, Direction::Down);
            }
            (Direction::Down, Entry::VerticalSplitter) => {
                self.traverse(row + 1, col, Direction::Down);
            }
            (Direction::Down, Entry::HorizontalSplitter) => {
                self.traverse(row, col - 1, Direction::Left);
                self.traverse(row, col + 1, Direction::Right);
            }
            (Direction::Down, Entry::LeftMirror) => {
                self.traverse(row, col + 1, Direction::Right);
            }
            (Direction::Down, Entry::RightMirror) => {
                self.traverse(row, col - 1, Direction::Left);
            }

            // left
            (Direction::Left, Entry::Empty) => {
                self.traverse(row, col - 1, Direction::Left);
            }
            (Direction::Left, Entry::VerticalSplitter) => {
                self.traverse(row - 1, col, Direction::Up);
                self.traverse(row + 1, col, Direction::Down);
            }
            (Direction::Left, Entry::HorizontalSplitter) => {
                self.traverse(row, col - 1, Direction::Left);
            }
            (Direction::Left, Entry::LeftMirror) => {
                self.traverse(row - 1, col, Direction::Up);
            }
            (Direction::Left, Entry::RightMirror) => {
                self.traverse(row + 1, col, Direction::Down);
            }
        }
    }

    fn energized(&self) -> usize {
        self.visited
            .iter()
            .filter_map(|(_, v)| (!v.is_empty()).then_some(1))
            .sum()
    }
}

pub fn part1() -> Result<()> {
    let input = include_str!("../../input/day16.txt");
    let grid = input.parse::<Grid>()?;
    tracing::debug!("grid:\n{}", grid);

    let mut traverser = Traverse::new(&grid);
    traverser.traverse(0, 0, Direction::Right);
    tracing::debug!("after traversal:\n{}", traverser);
    let part1 = traverser.energized();
    tracing::info!("[part 1] total tiles energized: {}", part1);

    let mut answers = vec![];
    for col in 0..grid.cols {
        for (row, dir) in [(0, Direction::Down), (grid.rows - 1, Direction::Up)].iter() {
            let mut traverser = Traverse::new(&grid);
            traverser.traverse(*row as isize, col as isize, *dir);
            let energized = traverser.energized();
            answers.push(energized);
        }
    }
    for row in 0..grid.rows {
        for (col, dir) in [(0, Direction::Right), (grid.cols - 1, Direction::Left)].iter() {
            let mut traverser = Traverse::new(&grid);
            traverser.traverse(row as isize, *col as isize, *dir);
            let energized = traverser.energized();
            answers.push(energized);
        }
    }

    let part2 = answers.into_iter().max().unwrap();
    tracing::info!("[part 2] max tiles energized: {}", part2);

    Ok(())
}

pub fn part2() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() -> Result<()> {
        let input = include_str!("../../input/day16.txt");
        let grid = input.parse::<Grid>()?;

        let mut traverser = Traverse::new(&grid);
        traverser.traverse(0, 0, Direction::Right);
        let part1 = traverser.energized();
        assert_eq!(part1, 46);
        Ok(())
    }
}
