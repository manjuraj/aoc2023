use anyhow::Result;
use std::{collections::HashSet, env};
use tracing::Level;

use aoc2023::{day01, day02, day03, day04, day05, day06, day07};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_max_level(Level::INFO)
        .with_line_number(true)
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

    if args.is_empty() || args.contains("3") {
        tracing::info!("Day 03");
        day03::part1_and_part2()?;
        tracing::info!("---");
    }

    if args.is_empty() || args.contains("4") {
        tracing::info!("Day 04");
        day04::part1_and_part2()?;
        tracing::info!("---");
    }

    if args.is_empty() || args.contains("5") {
        tracing::info!("Day 05");
        day05::part1_and_part2()?;
        tracing::info!("---");
    }

    if args.is_empty() || args.contains("6") {
        tracing::info!("Day 06");
        day06::part1_and_part2()?;
        tracing::info!("---");
    }

    if args.is_empty() || args.contains("7") {
        tracing::info!("Day 07");
        day07::part1_and_part2()?;
        tracing::info!("---");
    }

    Ok(())
}
