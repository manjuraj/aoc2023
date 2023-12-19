use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::Result;
use itertools::Itertools;

// Universe is a 2D grid of galaxies `[Galaxy]`.
// `[Galaxy]`` is a point in the grid encoding using its `(x, y)` coordinate.
// Moving horizontally to the right, incr x coordinate by 1.
// Moving vertically down, incr y coordinate by 1.
// Top left of the universe is (0, 0), borrow right is (max_x, max_y).

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Galaxy(usize, usize);

impl Galaxy {
    fn shortest_distance(&self, other: &Self) -> usize {
        let Galaxy(x1, y1) = self;
        let Galaxy(x2, y2) = other;
        (x1.max(x2) - x1.min(x2)) + (y1.max(y2) - y1.min(y2))
    }
}
#[derive(Debug, Default)]
struct Galaxies<'a>(HashSet<&'a Galaxy>);

impl<'a> Galaxies<'a> {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn insert(&mut self, galaxy: &'a Galaxy) {
        self.0.insert(galaxy);
    }
}

#[derive(Debug)]
struct Universe {
    galaxies: Vec<Galaxy>,
}

impl FromStr for Universe {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let galaxies = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| (c == '#').then_some(Galaxy(x, y)))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Universe { galaxies })
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let galaxies = self.galaxies.iter().collect::<HashSet<_>>();

        for y in 0..=self.max_rows() {
            for x in 0..=self.max_cols() {
                let galaxy = Galaxy(x, y);
                let c = if galaxies.contains(&galaxy) { '#' } else { '.' };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Universe {
    fn max_rows(&self) -> usize {
        self.galaxies.iter().map(|g| g.1).max().unwrap()
    }

    fn rows(&self) -> UniverseRowIter<'_> {
        UniverseRowIter::new(self)
    }

    fn expand_rows(&mut self) {
        let (new_rows, row_offsets) = self.rows().fold((0, vec![]), |(rows, mut acc), row| {
            if row.is_empty() {
                acc.push(rows);
                (rows + 1000000 - 1, acc)
            } else {
                acc.push(rows);
                (rows, acc)
            }
        });
        tracing::debug!("new rows added: {}", new_rows);
        tracing::debug!("row_offsets: {:?}", row_offsets);

        for galaxy in &mut self.galaxies {
            galaxy.1 += row_offsets[galaxy.1];
        }
    }

    fn max_cols(&self) -> usize {
        self.galaxies.iter().map(|g| g.0).max().unwrap()
    }

    fn cols(&self) -> UniverseColIter<'_> {
        UniverseColIter::new(self)
    }

    fn expand_cols(&mut self) {
        let (new_cols, col_offsets) = self.cols().fold((0, vec![]), |(cols, mut acc), col| {
            acc.push(cols);
            if col.is_empty() {
                (cols + 1000000 - 1, acc)
            } else {
                (cols, acc)
            }
        });
        tracing::debug!("new cols added: {}", new_cols);
        tracing::debug!("col_offsets: {:?}", col_offsets);

        for galaxy in &mut self.galaxies {
            galaxy.0 += col_offsets[galaxy.0];
        }
    }

    fn expand(&mut self) {
        self.expand_rows();
        self.expand_cols();
    }

    fn sum_of_shortest_distance(&self) -> usize {
        tracing::debug!("firing off!");
        self.galaxies
            .iter()
            .combinations(2)
            .inspect(|galaxies| tracing::debug!("galaxies: {:?}", galaxies))
            .map(|galaxies| {
                tracing::debug!("galaxies: {:?}", galaxies);
                let (g1, g2) = (galaxies[0], galaxies[1]);
                g1.shortest_distance(g2)
            })
            .sum::<usize>()
    }
}

#[derive(Debug)]
struct UniverseRowIter<'a> {
    #[allow(dead_code)]
    universe: &'a Universe,
    row_galaxies: HashMap<usize, Galaxies<'a>>,
    cur_row: usize,
    max_row: usize,
}

impl<'a> UniverseRowIter<'a> {
    fn new(universe: &'a Universe) -> Self {
        let max_row = universe.galaxies.iter().map(|g| g.1).max().unwrap();
        let row_galaxies =
            universe
                .galaxies
                .iter()
                .fold(HashMap::<usize, Galaxies>::new(), |mut acc, galaxy| {
                    acc.entry(galaxy.1).or_default().insert(galaxy);
                    acc
                });

        Self {
            universe,
            row_galaxies,
            cur_row: 0,
            max_row,
        }
    }
}

impl<'a> Iterator for UniverseRowIter<'a> {
    type Item = Galaxies<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(galaxies) = self.row_galaxies.remove(&self.cur_row) {
            self.cur_row += 1;
            Some(galaxies)
        } else if self.cur_row < self.max_row {
            self.cur_row += 1;
            Some(Galaxies::default())
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct UniverseColIter<'a> {
    #[allow(dead_code)]
    universe: &'a Universe,
    col_galaxies: HashMap<usize, Galaxies<'a>>,
    cur_col: usize,
    max_cols: usize,
}

impl<'a> UniverseColIter<'a> {
    fn new(universe: &'a Universe) -> Self {
        let col_galaxies =
            universe
                .galaxies
                .iter()
                .fold(HashMap::<usize, Galaxies>::new(), |mut acc, galaxy| {
                    acc.entry(galaxy.0).or_default().insert(galaxy);
                    acc
                });

        Self {
            universe,
            col_galaxies,
            cur_col: 0,
            max_cols: universe.max_cols(),
        }
    }
}

impl<'a> Iterator for UniverseColIter<'a> {
    type Item = Galaxies<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(galaxies) = self.col_galaxies.remove(&self.cur_col) {
            self.cur_col += 1;
            Some(galaxies)
        } else if self.cur_col < self.max_cols {
            self.cur_col += 1;
            Some(Galaxies::default())
        } else {
            None
        }
    }
}

pub fn part1_and_part2() -> Result<()> {
    let input = include_str!("../../input/day11.txt");
    let mut universe = input.parse::<Universe>()?;
    tracing::debug!("universe:\n{}", universe);
    for row in universe.rows() {
        tracing::debug!("row: {:?}", row);
    }

    // replace 1 by 1000000 for part 2
    universe.expand();
    // tracing::debug!("expanded universe:\n{}", universe);

    let part1 = universe.sum_of_shortest_distance();
    tracing::info!("[part 1 and 2] sum of shortet paths: {}", part1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample_day09() -> Result<()> {
        let input = include_str!("../../sample/day11.txt");
        let mut universe = input.parse::<Universe>()?;
        universe.expand();
        let part1 = universe.sum_of_shortest_distance();
        assert_eq!(part1, 374);
        Ok(())
    }
}
