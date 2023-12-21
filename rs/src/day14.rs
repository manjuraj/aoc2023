use core::fmt;
use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Entry {
    CubeRock,  // #
    RoundRock, // O
    Empty,     // .
}

impl TryFrom<u8> for Entry {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            b'#' => Ok(Entry::CubeRock),
            b'O' => Ok(Entry::RoundRock),
            b'.' => Ok(Entry::Empty),
            _ => anyhow::bail!("Invalid entry: {}", value),
        }
    }
}

// Grid is a 2D array of entries in *row-major* order.
// Horizontal rows, vertical columns.
// Moving down updates rows by 1, moving right updates columns by 1.
// Top-left is (0, 0), bottom-right is (rows - 1, cols - 1).
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
        writeln!(f, "{} x {}: ", self.rows, self.cols)?;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let entry = &self.entries[row][col];
                match entry {
                    Entry::CubeRock => write!(f, "#")?,
                    Entry::RoundRock => write!(f, "O")?,
                    Entry::Empty => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn tilt_north(&mut self) {
        for col in 0..self.cols {
            let mut start_row = 0;
            let mut cur_row = 0;
            let mut round_rocks = vec![];
            let mut empty = vec![];
            while cur_row < self.rows {
                match &self.entries[cur_row][col] {
                    Entry::CubeRock => {
                        // move the remaining entries to top of column
                        while let Some(entry) = round_rocks.pop() {
                            self.entries[start_row][col] = entry;
                            start_row += 1;
                        }
                        while let Some(entry) = empty.pop() {
                            self.entries[start_row][col] = entry;
                            start_row += 1;
                        }
                        // round_rocks and empty are now empty!
                        start_row += 1;
                    }
                    Entry::RoundRock => round_rocks.push(Entry::RoundRock),
                    Entry::Empty => empty.push(Entry::Empty),
                }
                cur_row += 1;
            }
            // move the remaining entries
            while let Some(entry) = round_rocks.pop() {
                self.entries[start_row][col] = entry;
                start_row += 1;
            }
            while let Some(entry) = empty.pop() {
                self.entries[start_row][col] = entry;
                start_row += 1;
            }
        }
    }

    fn tilt_west(&mut self) {
        for row in 0..self.rows {
            let mut start_col = 0;
            let mut cur_col = 0;
            let mut round_rocks = vec![];
            let mut empty = vec![];
            while cur_col < self.cols {
                match &self.entries[row][cur_col] {
                    Entry::CubeRock => {
                        // move the remaining entries to top of column
                        while let Some(entry) = round_rocks.pop() {
                            self.entries[row][start_col] = entry;
                            start_col += 1;
                        }
                        while let Some(entry) = empty.pop() {
                            self.entries[row][start_col] = entry;
                            start_col += 1;
                        }
                        // round_rocks and empty are now empty!
                        start_col += 1;
                    }
                    Entry::RoundRock => round_rocks.push(Entry::RoundRock),
                    Entry::Empty => empty.push(Entry::Empty),
                }
                cur_col += 1;
            }
            // move the remaining entries
            while let Some(entry) = round_rocks.pop() {
                self.entries[row][start_col] = entry;
                start_col += 1;
            }
            while let Some(entry) = empty.pop() {
                self.entries[row][start_col] = entry;
                start_col += 1;
            }
        }
    }

    fn tilt_south(&mut self) {
        for col in 0..self.cols {
            let mut start_row = self.rows - 1;
            let mut cur_row = self.rows - 1;
            let mut round_rocks = vec![];
            let mut empty = vec![];
            loop {
                match &self.entries[cur_row][col] {
                    Entry::CubeRock => {
                        // move the remaining entries to bottom of column
                        while let Some(entry) = round_rocks.pop() {
                            self.entries[start_row][col] = entry;
                            start_row = start_row.saturating_sub(1);
                        }
                        while let Some(entry) = empty.pop() {
                            self.entries[start_row][col] = entry;
                            start_row = start_row.saturating_sub(1);
                        }
                        // round_rocks and empty are now empty!
                        start_row = start_row.saturating_sub(1);
                    }
                    Entry::RoundRock => round_rocks.push(Entry::RoundRock),
                    Entry::Empty => empty.push(Entry::Empty),
                }
                if cur_row == 0 {
                    break;
                }
                cur_row -= 1;
            }
            // move the remaining entries
            while let Some(entry) = round_rocks.pop() {
                self.entries[start_row][col] = entry;
                start_row = start_row.saturating_sub(1);
            }
            while let Some(entry) = empty.pop() {
                self.entries[start_row][col] = entry;
                start_row = start_row.saturating_sub(1);
            }
        }
    }

    fn tilt_east(&mut self) {
        for row in 0..self.rows {
            let mut start_col = self.cols - 1;
            let mut cur_col = self.cols - 1;
            let mut round_rocks = vec![];
            let mut empty = vec![];
            loop {
                match &self.entries[row][cur_col] {
                    Entry::CubeRock => {
                        // move the remaining entries to right of row
                        while let Some(entry) = round_rocks.pop() {
                            self.entries[row][start_col] = entry;
                            start_col = start_col.saturating_sub(1);
                        }
                        while let Some(entry) = empty.pop() {
                            self.entries[row][start_col] = entry;
                            start_col = start_col.saturating_sub(1);
                        }
                        // round_rocks and empty are now empty!
                        start_col = start_col.saturating_sub(1);
                    }
                    Entry::RoundRock => round_rocks.push(Entry::RoundRock),
                    Entry::Empty => empty.push(Entry::Empty),
                }
                if cur_col == 0 {
                    break;
                }
                cur_col -= 1;
            }
            // move the remaining entries
            while let Some(entry) = round_rocks.pop() {
                self.entries[row][start_col] = entry;
                start_col = start_col.saturating_sub(1);
            }
            while let Some(entry) = empty.pop() {
                self.entries[row][start_col] = entry;
                start_col = start_col.saturating_sub(1);
            }
        }
    }

    fn load(&self) -> usize {
        let mut sum = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let entry = &self.entries[row][col];
                match entry {
                    Entry::CubeRock => {}
                    Entry::RoundRock => sum += self.rows - row,
                    Entry::Empty => {}
                }
            }
        }
        sum
    }
}

pub fn part1() -> Result<()> {
    let input = include_str!("../../input/day14.txt");
    let mut grid = input.parse::<Grid>()?;
    tracing::debug!("original grid:\n{}", grid);
    grid.tilt_north();
    tracing::debug!("grid after being tilted north:\n{}", grid);
    let part1 = grid.load();
    tracing::debug!("[part 1] total load contributed by round rocks: {}", part1);
    Ok(())
}

pub fn part2() -> Result<()> {
    let input = include_str!("../../input/day14.txt");
    let mut grid = input.parse::<Grid>()?;
    tracing::debug!("original grid:\n{}", grid);

    let mut loads = vec![];
    for i in 0..1000 {
        grid.tilt_north();
        grid.tilt_west();
        grid.tilt_south();
        grid.tilt_east();
        let load = grid.load();
        // tracing::debug!("grid after {} cycle has load {}:\n{}", i, grid.load(), grid);
        tracing::debug!("grid after {} cycle has load {}", i + 1, load);
        loads.push(load);
    }
    tracing::debug!("loads: {:?}", loads);
    tracing::debug!("{}", (1000000000 - 2) % 7);

    // Repeats after 97 loads
    // Repeating cycle is: 96345, 96340, 96317, 96293, 96297, 96314, 96325, 96333, 96344
    // (1000000000 - 97)%9 = 3
    // So answer is: 96317

    // answer found by inspection!

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample() -> Result<()> {
        Ok(())
    }
}
