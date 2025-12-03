use aoc_2025::solution_for;
use clap::Parser;

#[derive(Parser)]
struct Args {
    day: u32,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let input = std::fs::read_to_string(&format!("./inputs/{:02}.txt", args.day))?;
    let (part1, part2) = solution_for(args.day)?;

    let r1 = part1(&input)?;
    println!("Part 1: {r1}");

    let r2 = part2(&input)?;
    println!("Part 2: {r2}");

    Ok(())
}
