use anyhow::Result;

// Grid that operates on a 2D array of tiles as:
// - Move left is x - 1
// - Move right is x + 1
// - Move up is y - 1
// - Move down is y + 1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos(usize, usize);

impl Pos {
    fn left(&self) -> Self {
        let Pos(x, y) = self;
        Pos(x - 1, *y)
    }

    fn right(&self) -> Self {
        let Pos(x, y) = self;
        Pos(x + 1, *y)
    }

    fn up(&self) -> Self {
        let Pos(x, y) = self;
        Pos(*x, y - 1)
    }

    fn down(&self) -> Self {
        let Pos(x, y) = self;
        Pos(*x, y + 1)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Vertical,   // |
    Horizontal, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Ground,     // .
    Start,      // S
}

impl TryFrom<u8> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            b'|' => Ok(Self::Vertical),
            b'-' => Ok(Self::Horizontal),
            b'L' => Ok(Self::NorthEast),
            b'J' => Ok(Self::NorthWest),
            b'7' => Ok(Self::SouthWest),
            b'F' => Ok(Self::SouthEast),
            b'.' => Ok(Self::Ground),
            b'S' => Ok(Self::Start),
            _ => Err(anyhow::anyhow!("invalid tile: {}", value)),
        }
    }
}

#[derive(Debug)]
struct Sketch {
    tiles: Vec<Vec<Tile>>,
    start: Pos,
}

impl Sketch {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let start = tiles
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .find_map(|(x, &tile)| (tile == Tile::Start).then_some(Pos(x, y)))
            })
            .expect("no start tile found");
        Self { tiles, start }
    }

    fn get(&self, pos: &Pos) -> Option<Tile> {
        let &Pos(x, y) = pos;
        self.tiles.get(y).and_then(|row| row.get(x)).copied()
    }

    fn visit(&self, pos: &Pos, dir: Direction, steps: usize) -> Option<usize> {
        assert!(self.get(pos).is_some());
        tracing::debug!("visiting {:?} {:?} {}", pos, dir, steps);

        match dir {
            Direction::North => self.get(&pos.up()).and_then(|tile| match tile {
                Tile::Vertical => self.visit(&pos.up(), Direction::North, steps + 1),
                Tile::SouthEast => self.visit(&pos.up(), Direction::East, steps + 1),
                Tile::SouthWest => self.visit(&pos.up(), Direction::West, steps + 1),
                Tile::Start => Some(steps + 1),
                _ => {
                    tracing::debug!("backtracking");
                    None
                }
            }),
            Direction::South => self.get(&pos.down()).and_then(|tile| match tile {
                Tile::Vertical => self.visit(&pos.down(), Direction::South, steps + 1),
                Tile::NorthEast => self.visit(&pos.down(), Direction::East, steps + 1),
                Tile::NorthWest => self.visit(&pos.down(), Direction::West, steps + 1),
                Tile::Start => Some(steps + 1),
                _ => {
                    tracing::debug!("backtracking");
                    None
                }
            }),
            Direction::East => self.get(&pos.right()).and_then(|tile| match tile {
                Tile::Horizontal => self.visit(&pos.right(), Direction::East, steps + 1),
                Tile::NorthWest => self.visit(&pos.right(), Direction::North, steps + 1),
                Tile::SouthWest => self.visit(&pos.right(), Direction::South, steps + 1),
                Tile::Start => Some(steps + 1),
                _ => {
                    tracing::debug!("backtracking");
                    None
                }
            }),
            Direction::West => self.get(&pos.left()).and_then(|tile| match tile {
                Tile::Horizontal => self.visit(&pos.left(), Direction::West, steps + 1),
                Tile::NorthEast => self.visit(&pos.left(), Direction::North, steps + 1),
                Tile::SouthEast => self.visit(&pos.left(), Direction::South, steps + 1),
                Tile::Start => Some(steps + 1),
                _ => {
                    tracing::debug!("backtracking");
                    None
                }
            }),
        }
    }
}

pub fn part1_and_part2() -> Result<()> {
    let tiles = include_bytes!("../../input/day10.txt")
        .split(|&b| b == b'\n')
        .map(|line| {
            line.iter()
                .map(|&b| Tile::try_from(b))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;
    let sketch = Sketch::new(tiles);
    tracing::debug!("{:?}", sketch);

    let steps = sketch
        .get(&sketch.start.right())
        .and_then(|tile| match tile {
            Tile::Horizontal => sketch.visit(&sketch.start.right(), Direction::East, 0),
            Tile::NorthWest => sketch.visit(&sketch.start.right(), Direction::North, 0),
            Tile::SouthWest => sketch.visit(&sketch.start.right(), Direction::South, 0),
            _ => None,
        })
        .or_else(|| {
            sketch
                .get(&sketch.start.left())
                .and_then(|tile| match tile {
                    Tile::Horizontal => sketch.visit(&sketch.start.left(), Direction::West, 0),
                    Tile::NorthEast => sketch.visit(&sketch.start.left(), Direction::North, 0),
                    Tile::SouthEast => sketch.visit(&sketch.start.left(), Direction::South, 0),
                    _ => None,
                })
        })
        .or_else(|| {
            sketch.get(&sketch.start.up()).and_then(|tile| match tile {
                Tile::Vertical => sketch.visit(&sketch.start.up(), Direction::North, 0),
                Tile::SouthEast => sketch.visit(&sketch.start.up(), Direction::East, 0),
                Tile::SouthWest => sketch.visit(&sketch.start.up(), Direction::West, 0),
                _ => None,
            })
        })
        .or_else(|| {
            sketch
                .get(&sketch.start.down())
                .and_then(|tile| match tile {
                    Tile::Vertical => sketch.visit(&sketch.start.down(), Direction::South, 0),
                    Tile::NorthEast => sketch.visit(&sketch.start.down(), Direction::East, 0),
                    Tile::NorthWest => sketch.visit(&sketch.start.down(), Direction::West, 0),
                    _ => None,
                })
        })
        .expect("no path found");

    tracing::info!("[part 1]: farthest point is {} steps away", steps / 2 + 1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_sample_day09() -> Result<()> {
        Ok(())
    }
}
