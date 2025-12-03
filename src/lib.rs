pub mod day01;
pub mod day02;

pub type Solver = (
    fn(&str) -> Result<String, anyhow::Error>,
    fn(&str) -> Result<String, anyhow::Error>,
);

pub fn solution_for(day: u32) -> Result<Solver, anyhow::Error> {
    match day {
        1 => Ok((day01::part1, day01::part2)),
        2 => Ok((day02::part1, day02::part2)),
        _ => anyhow::bail!("No solution found for day {day}"),
    }
}
