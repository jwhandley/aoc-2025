use itertools::Itertools;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let banks = input.lines();
    let mut total = 0;
    for bank in banks {
        let first = bank[..(bank.len() - 1)].chars().max().unwrap();
        let (idx, _) = bank.chars().find_position(|&c| c == first).unwrap();

        let last = bank[(idx + 1)..].chars().max().unwrap();

        total += first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap();
    }

    Ok(total.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let banks = input.lines();
    let mut total = 0;
    for bank in banks {
        let mut v: u64 = 0;
        let mut start = 0;
        for i in 0..12 {
            let end = bank.len() - 11 + i;

            let first = bank[start..end].chars().max().unwrap();
            let (idx, _) = bank[start..]
                .chars()
                .find_position(|&c| c == first)
                .unwrap();

            v = v * 10 + first.to_digit(10).unwrap() as u64;
            start = start + idx + 1;
        }

        total += v;
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../samples/03.txt");
        if let Ok(result) = part1(input) {
            assert_eq!(result, "357".to_string());
        }
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../samples/03.txt");
        if let Ok(result) = part2(input) {
            assert_eq!(result, "3121910778619".to_string());
        }
    }
}
