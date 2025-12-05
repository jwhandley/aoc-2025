use std::ops::RangeInclusive;

use anyhow::anyhow;
use itertools::Itertools;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{self, char, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
};

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let (_, (ranges, ids)) = parse_input
        .parse(input)
        .map_err(|e| anyhow!("Failed to parse input {e}"))?;

    let answer = ids
        .iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count();

    Ok(answer.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let (_, (ranges, _)) = parse_input
        .parse(input)
        .map_err(|e| anyhow!("Failed to parse input {e}"))?;

    let (_, total) = ranges
        .iter()
        .sorted_by_key(|r| *r.start())
        .map(|r| (r.clone(), r.try_len().unwrap()))
        .reduce(
            |(current, total), (range, length)| match current.contains(range.start()) {
                true if range.end() > current.end() => (
                    *current.start()..=*range.end(),
                    total + *range.end() as usize - *current.end() as usize,
                ),
                false => (range, total + length),
                _ => (current, total),
            },
        )
        .unwrap();

    Ok(total.to_string())
}

fn parse_input(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    separated_pair(
        separated_list1(
            line_ending,
            separated_pair(complete::u64, char('-'), complete::u64).map(|(start, end)| start..=end),
        ),
        tag("\n\n"),
        separated_list1(line_ending, complete::u64),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../../samples/05.txt");

    #[test]
    fn part1_test() -> Result<(), anyhow::Error> {
        assert_eq!(part1(INPUT)?, "3");
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<(), anyhow::Error> {
        assert_eq!(part2(INPUT)?, "14");
        Ok(())
    }
}
