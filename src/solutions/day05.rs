use anyhow::anyhow;
use chumsky::prelude::*;
use itertools::Itertools;
use std::ops::RangeInclusive;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let (ranges, ids) = parse_input()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow!("Failed to parse input {e:?}"))?;

    let answer = ids
        .iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count();

    Ok(answer.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let (ranges, _) = parse_input()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow!("Failed to parse input {e:?}"))?;

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

fn parse_input<'src>() -> impl Parser<'src, &'src str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let num = text::int(10).map(|v: &str| v.parse::<u64>().unwrap());
    let ranges = num
        .then_ignore(just('-'))
        .then(num)
        .map(|(start, end)| start..=end)
        .separated_by(text::newline())
        .collect();

    let ids = num.separated_by(text::newline()).collect();

    ranges.then_ignore(just("\n\n")).then(ids)
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
