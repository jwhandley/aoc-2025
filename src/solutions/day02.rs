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
        let length = n.ilog10() + 1;
        (1..=(length / 2)).any(|size| length % size == 0 && chunk_number(n, size).all_equal())
    }

    Ok(ranges.to_string())
}

fn chunk_number(n: u64, by: u32) -> impl Iterator<Item = u64> {
    let divisor = 10u64.pow(by);
    let total = n.ilog10() / by + 1;

    (0..total).scan(n, move |current, _| {
        let r = *current % divisor;
        *current /= divisor;
        Some(r)
    })
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

    #[test]
    fn chunk_number_test() {
        assert_eq!(chunk_number(222, 1).collect::<Vec<_>>(), vec![2, 2, 2]);
        assert_eq!(chunk_number(2222, 2).collect::<Vec<_>>(), vec![22, 22]);
        assert_eq!(chunk_number(998, 2).collect::<Vec<_>>(), vec![98, 9]);
        assert_eq!(chunk_number(11, 1).collect::<Vec<_>>(), vec![1, 1]);
    }
}
