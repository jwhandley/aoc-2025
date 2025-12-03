use itertools::Itertools;
use rayon::prelude::*;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let ranges: u64 = input
        .split(',')
        .map(|p| {
            let (a, b) = p.split_once('-').unwrap();
            let a: u64 = a.parse().unwrap();
            let b: u64 = b.parse().unwrap();

            a..=b
        })
        .par_bridge()
        .map(|r| r.filter(|&v| is_invalid_id(v)).sum::<u64>())
        .sum();

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
    let ranges: u64 = input
        .split(',')
        .map(|p| {
            let (a, b) = p.split_once('-').unwrap();
            let a: u64 = a.parse().unwrap();
            let b: u64 = b.parse().unwrap();

            a..=b
        })
        .par_bridge()
        .map(|r| r.filter(|&v| is_invalid_id(v)).sum::<u64>())
        .sum();

    fn is_invalid_id(n: u64) -> bool {
        let s = n.to_string();
        let length = s.len();

        for size in (1..=(length / 2)).rev() {
            if s.as_bytes().chunks(size).all_equal() {
                return true;
            }
        }

        false
    }

    Ok(ranges.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../samples/02.txt");

        if let Ok(result) = part1(input) {
            assert_eq!(result, "1227775554".to_string())
        }
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../samples/02.txt");

        if let Ok(result) = part2(input) {
            assert_eq!(result, "4174379265".to_string())
        }
    }
}
