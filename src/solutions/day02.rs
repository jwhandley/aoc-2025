use anyhow::anyhow;
use chumsky::prelude::*;
use itertools::Itertools;
use rayon::prelude::*;
use std::ops::RangeInclusive;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let ranges = ranges()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow!("Failed to parse input {e:?}"))?;

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
    let ranges = ranges()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow!("Failed to parse input {e:?}"))?;

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

fn ranges<'src>() -> impl Parser<'src, &'src str, Vec<RangeInclusive<u64>>> {
    let uint64 = text::int(10).map(|v: &str| v.parse::<u64>().unwrap());

    let pair = uint64
        .then_ignore(just('-'))
        .then(uint64)
        .map(|(start, end)| start..=end);

    pair.separated_by(just(',')).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() -> anyhow::Result<()> {
        let input = include_str!("../../samples/02.txt");

        assert_eq!(part1(input)?, "1227775554".to_string());
        Ok(())
    }

    #[test]
    fn part2_test() -> anyhow::Result<()> {
        let input = include_str!("../../samples/02.txt");

        assert_eq!(part2(input)?, "4174379265".to_string());
        Ok(())
    }
}
