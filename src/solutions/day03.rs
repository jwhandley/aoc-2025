pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let total = input
        .lines()
        .map(|s| s.as_bytes())
        .map(|bank| solve(bank, 2))
        .sum::<u64>();

    Ok(total.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let total = input
        .lines()
        .map(|s| s.as_bytes())
        .map(|bank| solve(bank, 12))
        .sum::<u64>();

    Ok(total.to_string())
}

fn solve(bank: &[u8], n: usize) -> u64 {
    let (value, _) = (0..n).fold((0, 0), |(v, start), i| {
        let end = bank.len() - (n - 1) + i;

        let (idx, first) = max_index(&bank[start..end]).unwrap();
        let n = (*first - b'0') as u64;

        (v * 10 + n, start + idx + 1)
    });

    value
}

fn max_index<T: Ord>(xs: &[T]) -> Option<(usize, &T)> {
    xs.iter()
        .enumerate()
        .reduce(|acc, next| if next.1 > acc.1 { next } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../../samples/03.txt");

    #[test]
    fn part1_test() {
        if let Ok(result) = part1(INPUT) {
            assert_eq!(result, "357".to_string());
        }
    }

    #[test]
    fn part2_test() {
        if let Ok(result) = part2(INPUT) {
            assert_eq!(result, "3121910778619".to_string());
        }
    }
}
