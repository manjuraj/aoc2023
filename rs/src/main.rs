use anyhow::Result;
use std::{collections::HashSet, env};
use tracing::Level;

use aoc2023::{day01, day02};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_max_level(Level::INFO)
        .compact()
        .init();

    let args = env::args().skip(1).collect::<HashSet<_>>();

    if args.is_empty() || args.contains("1") {
        tracing::info!("Day 01");
        day01::part1()?;
        day01::part2()?;
        tracing::info!("---");
    }

    if args.is_empty() || args.contains("2") {
        tracing::info!("Day 02");
        day02::part1_and_part2()?;
        tracing::info!("---");
    }

    Ok(())
}
