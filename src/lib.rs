mod solutions;
pub mod utils;
use crate::solutions::*;

pub type Solver = (
    fn(&str) -> Result<String, anyhow::Error>,
    fn(&str) -> Result<String, anyhow::Error>,
);

pub fn solution_for(day: u32) -> Result<Solver, anyhow::Error> {
    match day {
        1 => Ok((day01::part1, day01::part2)),
        2 => Ok((day02::part1, day02::part2)),
        3 => Ok((day03::part1, day03::part2)),
        4 => Ok((day04::part1, day04::part2)),
        5 => Ok((day05::part1, day05::part2)),
        6 => Ok((day06::part1, day06::part2)),
        7 => Ok((day07::part1, day07::part2)),
        8 => Ok((day08::part1, day08::part2)),
        9 => Ok((day09::part1, day09::part2)),
        10 => Ok((day10::part1, day10::part2)),
        _ => anyhow::bail!("No solution found for day {day}"),
    }
}
