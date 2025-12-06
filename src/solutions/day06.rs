use itertools::Itertools;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let lines: Vec<_> = input.lines().collect();
    let instructions: Vec<_> = lines[lines.len() - 1]
        .split_ascii_whitespace()
        .map(|c| match c {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("Unexpected token {c}"),
        })
        .collect();

    let nums: Vec<Vec<u64>> = lines
        .iter()
        .take(lines.len() - 1)
        .map(|l| {
            l.split_ascii_whitespace()
                .flat_map(|s| s.parse::<u64>())
                .collect()
        })
        .collect();

    let nums = transpose(nums);

    let result: u64 = nums
        .iter()
        .enumerate()
        .filter_map(|(i, xs)| {
            let op = &instructions[i];
            xs.into_iter().copied().reduce(|acc, next| match op {
                Op::Add => acc + next,
                Op::Mul => acc * next,
            })
        })
        .sum();

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let lines: Vec<_> = input.lines().collect();
    let instructions: Vec<_> = lines[lines.len() - 1]
        .split_ascii_whitespace()
        .map(|c| match c {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("Unexpected token {c}"),
        })
        .collect();

    let v: Vec<Vec<_>> = lines[..lines.len() - 1]
        .iter()
        .map(|l| l.chars().collect())
        .collect();

    let t = transpose(v.clone());
    let t: Vec<_> = t
        .iter()
        .map(|c| c.iter().collect::<String>().trim().to_string())
        .map(|s| s.parse::<u64>())
        .chunk_by(|n| n.is_err())
        .into_iter()
        .filter(|g| !g.0)
        .map(|g| g.1.map(|x| x.unwrap()).collect_vec())
        .collect();

    let result: u64 = t
        .iter()
        .enumerate()
        .filter_map(|(i, xs)| {
            let op = &instructions[i];
            xs.into_iter().copied().reduce(|acc, next| match op {
                Op::Add => acc + next,
                Op::Mul => acc * next,
            })
        })
        .sum();

    Ok(result.to_string())
}

fn transpose<T: Copy>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    (0..len)
        .map(|i| v.iter().map(|row| row[i]).collect())
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add,
    Mul,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() -> anyhow::Result<()> {
        let input = include_str!("../../samples/06.txt");
        assert_eq!(part1(input)?, "4277556".to_string());
        Ok(())
    }

    #[test]
    fn part2_test() -> anyhow::Result<()> {
        let input = include_str!("../../samples/06.txt");
        assert_eq!(part2(input)?, "3263827".to_string());
        Ok(())
    }
}
