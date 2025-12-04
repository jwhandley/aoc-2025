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
        _ => anyhow::bail!("No solution found for day {day}"),
    }
}
