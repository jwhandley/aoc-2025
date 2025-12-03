use anyhow::Context;
use aoc_2025::solution_for;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action {
    Day { day: u32 },
    All,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    match args.command {
        Action::Day { day } => solve_day(day),
        Action::All => {
            for day in 1..=25 {
                if let Ok((part1, part2)) = solution_for(day) {
                    let input = std::fs::read_to_string(format!("./inputs/{:02}.txt", day))
                        .with_context(|| format!("Unable to find input for day {}", day))?;

                    println!("Solving day {day}");
                    let now = std::time::Instant::now();
                    let r1 = part1(&input)?;
                    println!("Part 1: {r1}");

                    let r2 = part2(&input)?;
                    println!("Part 2: {r2}");

                    println!("Solved in {:>10}", format!("{:.02?}", now.elapsed()));
                    println!("--------------------");
                }
            }

            Ok(())
        }
    }
}

fn solve_day(day: u32) -> Result<(), anyhow::Error> {
    let input = std::fs::read_to_string(format!("./inputs/{:02}.txt", day))
        .with_context(|| format!("Unable to find input for day {}", day))?;
    let (part1, part2) = solution_for(day)?;

    let now = std::time::Instant::now();
    let r1 = part1(&input)?;
    println!("Part 1: {r1}");

    let r2 = part2(&input)?;
    println!("Part 2: {r2}");

    println!("Solved in {:?}", now.elapsed());

    Ok(())
}
