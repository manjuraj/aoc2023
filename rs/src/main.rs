use anyhow::Result;
use std::{collections::HashSet, env};
use tracing::Level;

use aoc2023::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day13, day14,
    day15,
};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_max_level(Level::DEBUG)
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

    if args.is_empty() || args.contains("8") {
        tracing::info!("Day 08");
        day08::part1_and_part2()?;
        tracing::info!("---");
    }

    if args.is_empty() || args.contains("9") {
        tracing::info!("Day 09");
        day09::part1_and_part2()?;
        tracing::info!("---");
    }

    if args.is_empty() || args.contains("10") {
        tracing::info!("Day 10");
        day10::part1_and_part2()?;
        tracing::info!("---");
    }

    if args.is_empty() || args.contains("11") {
        tracing::info!("Day 11");
        day11::part1_and_part2()?;
        tracing::info!("---");
    }

    if args.is_empty() || args.contains("13") {
        tracing::info!("Day 13");
        day13::part1_and_part2()?;
        tracing::info!("---");
    }

    if args.is_empty() || args.contains("14") {
        tracing::info!("Day 14");
        day14::part1()?;
        day14::part2()?;
        tracing::info!("---");
    }

    if args.is_empty() || args.contains("15") {
        tracing::info!("Day 15");
        day15::part1()?;
        day15::part2()?;
        tracing::info!("---");
    }

    Ok(())
}
