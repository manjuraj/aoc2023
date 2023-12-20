use core::fmt;
use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
enum Entry {
    Ash,
    Rock,
}

impl TryFrom<u8> for Entry {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self> {
        Ok(match value {
            b'#' => Entry::Rock,
            b'.' => Entry::Ash,
            _ => anyhow::bail!("Invalid entry: {}", value),
        })
    }
}

// Pattern grid
// Update x-coordinate to move down/up
// Update y-coordinate to move right/left
#[derive(Debug)]
struct Pattern {
    grid: Vec<Vec<Entry>>,
    rows: usize,
    cols: usize,
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} x {}:", self.rows, self.cols)?;
        for row in &self.grid {
            for entry in row {
                match entry {
                    Entry::Ash => write!(f, ".")?,
                    Entry::Rock => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TryFrom<&[u8]> for Pattern {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let grid = value
            .split(|&c| c == b'\n')
            .map(|line| {
                line.iter()
                    .map(|&b| b.try_into())
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<Vec<_>>>>()?;
        let rows = grid.len();
        let cols = grid[0].len();
        Ok(Pattern { grid, rows, cols })
    }
}

impl Pattern {
    // Find a vertial reflection line between column index `(mid, mid + 1)`
    // If the pattern across the middle line is symmetric, then return true,
    // otherwise false
    fn is_vertical_reflection_line_at(&self, mid: usize) -> bool {
        let lb = 0;
        let ub = self.cols - 1;
        // NB mid and mid + 1 must be valid. Hence, mid < ub
        assert!(lb <= mid && mid < ub);
        // At least one column to the left and right of the middle line
        assert!(ub - lb >= 1);
        // Length of the reflection to check
        let len = usize::min(mid - lb + 1, ub - (mid + 1) + 1);

        // // >> PART 1 start
        // for row in 0..self.rows {
        //     // for each row, check [lb, mid] and [mid + 1, ub]
        //     let mut ub1 = mid;
        //     let mut lb2 = mid + 1;
        //     for i in 0..len {
        //         assert!(lb <= ub1 && lb2 <= ub);
        //         assert!(ub1 < self.cols && lb2 < self.cols);
        //         if self.grid[row][ub1] != self.grid[row][lb2] {
        //             return false;
        //         }
        //         assert!(ub1 > 0 || i == len - 1);
        //         ub1 = ub1.saturating_sub(1);
        //         lb2 += 1;
        //     }
        // }
        // true
        // // PART 1 end <<

        // >> PART 2 start
        // for each row, check [lb, mid] and [mid + 1, ub]
        let mut ub1 = mid;
        let mut lb2 = mid + 1;
        let mut different = 0;
        for i in 0..len {
            let mut differnet_across_rows = 0;
            for row in 0..self.rows {
                assert!(lb <= ub1 && lb2 <= ub);
                assert!(ub1 < self.cols && lb2 < self.cols);
                if self.grid[row][ub1] != self.grid[row][lb2] {
                    differnet_across_rows += 1;
                }
            }

            // short circuit
            if differnet_across_rows > 1 {
                return false;
            }

            if differnet_across_rows == 1 {
                different += 1;
            }

            // short cicuit
            if different > 1 {
                return false;
            }

            assert!(ub1 > 0 || i == len - 1);
            ub1 = ub1.saturating_sub(1);
            lb2 += 1;
        }
        different == 1
        // PART 2 end <<
    }

    fn vertical_reflection_line(&self, lb: usize, ub: usize) -> Option<usize> {
        if lb >= ub {
            return None;
        }

        let mid = (lb + ub) / 2;
        if self.is_vertical_reflection_line_at(mid) {
            Some(mid)
        } else {
            self.vertical_reflection_line(mid + 1, ub)
                .or_else(|| self.vertical_reflection_line(lb, mid))
        }
    }

    fn vertical_line(&self) -> Option<usize> {
        tracing::debug!("exploring vertical reflection line");
        let res = self.vertical_reflection_line(0, self.cols - 1);
        tracing::debug!("found vertical reflection line: {:?}", res);
        res
    }

    // Find a horizontal reflection line between column index `mid` and
    // `mid + 1`. If the pattern is symmetric, then return true,
    // otherwise false
    fn is_horizontal_reflection_line_at(&self, mid: usize) -> bool {
        tracing::debug!("checking horizontal reflection line at {}", mid);
        let lb = 0;
        let ub = self.rows - 1;
        // NB mid and mid + 1 must be valid. Hence, mid < ub
        assert!(lb <= mid && mid < ub);
        // At least one row to the top and bottom of the middle line
        assert!(ub - lb >= 1);
        // Length of the reflection to check
        let len = usize::min(mid - lb + 1, ub - (mid + 1) + 1);

        // // PART 1 start >>
        // for col in 0..self.cols {
        //     let mut ub1 = mid;
        //     let mut lb2 = mid + 1;
        //     for i in 0..len {
        //         assert!(lb <= ub1 && lb2 <= ub);
        //         assert!(ub1 < self.rows && lb2 < self.rows);
        //         if self.grid[ub1][col] != self.grid[lb2][col] {
        //             return false;
        //         }
        //         assert!(ub1 > 0 || i == len - 1);
        //         ub1 = ub1.saturating_sub(1);
        //         lb2 += 1;
        //     }
        // }
        // true
        // // PART 1 end <<

        // PART 2 start >>
        let mut ub1 = mid;
        let mut lb2 = mid + 1;
        let mut different = 0;
        for i in 0..len {
            tracing::debug!(
                "checking window: {}, ({}, {}) <> ({}, {}), different: {}",
                i,
                lb,
                ub1,
                lb2,
                ub,
                different
            );

            let mut different_across_cols = 0;
            for col in 0..self.cols {
                assert!(lb <= ub1 && lb2 <= ub);
                assert!(ub1 < self.rows && lb2 < self.rows);
                if self.grid[ub1][col] != self.grid[lb2][col] {
                    different_across_cols += 1;
                }

                // short circuit
                if different_across_cols > 1 {
                    tracing::debug!("short circuit");
                    return false;
                }
            }

            if different_across_cols == 1 {
                different += 1;
                tracing::debug!(
                    "different: {}, different_across_cols: {}",
                    different,
                    different_across_cols
                );
            }

            // short cicuit
            if different > 1 {
                return false;
            }
            assert!(ub1 > 0 || i == len - 1);
            ub1 = ub1.saturating_sub(1);
            lb2 += 1;
        }
        different == 1
        // PART 2 end <<
    }

    fn horizontal_reflection_line(&self, lb: usize, ub: usize) -> Option<usize> {
        if lb == ub {
            return None;
        }

        let mid = (lb + ub) / 2;
        if self.is_horizontal_reflection_line_at(mid) {
            Some(mid)
        } else {
            self.horizontal_reflection_line(mid + 1, ub)
                .or_else(|| self.horizontal_reflection_line(lb, mid))
        }
    }

    fn horizontal_line(&self) -> Option<usize> {
        tracing::debug!("exploring horizontal reflection line");
        let res = self.horizontal_reflection_line(0, self.rows - 1);
        tracing::debug!("found horizontal reflection line: {:?}", res);
        res
    }
}

#[derive(Debug)]
struct Patterns(Vec<Pattern>);

impl FromStr for Patterns {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let patterns = s
            .split("\n\n")
            .map(|s| s.as_bytes())
            .map(Pattern::try_from)
            .collect::<Result<Vec<_>>>()?;
        Ok(Patterns(patterns))
    }
}

pub fn part1_and_part2() -> Result<()> {
    let input = include_str!("../../input/day13.txt");
    let patterns = input.parse::<Patterns>()?;
    let mut sum = 0;
    for pattern in &patterns.0 {
        tracing::debug!("pattern:\n{}", pattern);

        if let Some(mid) = pattern.vertical_line() {
            tracing::debug!(
                "vertical reflection line between column: {} and {}, with {} columns to left",
                mid,
                mid + 1,
                mid + 1
            );

            sum += mid + 1;
        }

        if let Some(mid) = pattern.horizontal_line() {
            tracing::debug!(
                "horizontal reflection line between row: {} and {}, with {} row to top",
                mid,
                mid + 1,
                mid + 1
            );

            sum += 100 * (mid + 1);
        }
    }
    tracing::info!("[part 1] sum: {}", sum);

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
