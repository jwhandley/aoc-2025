use itertools::Itertools;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let (a, b) = input.split_once("\n\n").unwrap();
    let ranges: Vec<_> = a
        .lines()
        .map(|s| {
            let (a, b) = s.split_once('-').unwrap();
            let a: u64 = a.parse().unwrap();
            let b: u64 = b.parse().unwrap();

            a..=b
        })
        .collect();

    let ids = b.lines().flat_map(|s| s.parse::<u64>());

    let answer = ids
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count();

    Ok(answer.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let (a, _) = input.split_once("\n\n").unwrap();
    let mut ranges = a
        .lines()
        .map(|s| {
            let (a, b) = s.split_once('-').unwrap();
            let a: usize = a.parse().unwrap();
            let b: usize = b.parse().unwrap();

            a..=b
        })
        .sorted_by_key(|r| *r.start());

    let mut total: usize = 0;
    let mut current = ranges.next().unwrap();

    for range in ranges {
        if current.contains(range.start()) && range.end() > current.end() {
            current = *current.start()..=*range.end();
        } else if !current.contains(range.start()) {
            total += current.try_len().unwrap();
            current = range.clone();
        }
    }

    total += current.try_len().unwrap();

    Ok(total.to_string())
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
