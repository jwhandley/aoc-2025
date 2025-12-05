use std::ops::RangeInclusive;

use itertools::Itertools;
use nom::{
    IResult, Parser, bytes::complete::tag, character::complete, multi::separated_list1,
    sequence::separated_pair,
};

use rayon::prelude::*;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let (_, ranges) = ranges.parse(input).unwrap();

    let ranges = ranges
        .into_par_iter()
        .map(|r| r.filter(|&v| is_invalid_id(v)).sum::<u64>())
        .sum::<u64>();

    fn is_invalid_id(n: u64) -> bool {
        let length = n.ilog10() + 1;
        if length % 2 != 0 {
            return false;
        }

        let divisor = 10u64.pow(length / 2);

        let first = n / divisor;
        let rest = n % divisor;

        first == rest
    }

    Ok(ranges.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let (_, ranges) = ranges.parse(input).unwrap();

    let ranges = ranges
        .into_par_iter()
        .map(|r| r.filter(|&v| is_invalid_id(v)).sum::<u64>())
        .sum::<u64>();

    fn is_invalid_id(n: u64) -> bool {
        let s = n.to_string();
        let length = s.len();

        for size in (1..=(length / 2)).rev() {
            if length % size != 0 {
                continue;
            }

            if s.as_bytes().chunks(size).all_equal() {
                return true;
            }
        }

        false
    }

    Ok(ranges.to_string())
}

fn ranges(input: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list1(
        tag(","),
        separated_pair(complete::u64, tag("-"), complete::u64).map(|(start, end)| start..=end),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../../samples/02.txt");

        if let Ok(result) = part1(input) {
            assert_eq!(result, "1227775554".to_string())
        }
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../../samples/02.txt");

        if let Ok(result) = part2(input) {
            assert_eq!(result, "4174379265".to_string())
        }
    }
}
