pub mod day01;

pub type Solver = (
    fn(&str) -> Result<String, anyhow::Error>,
    fn(&str) -> Result<String, anyhow::Error>,
);

pub fn solution_for(day: u32) -> Result<Solver, anyhow::Error> {
    match day {
        1 => Ok((day01::part1, day01::part2)),
        _ => anyhow::bail!("No solution found for day {day}"),
    }
}
