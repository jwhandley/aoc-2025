use anyhow::anyhow;
use chumsky::{
    prelude::*,
    text::{inline_whitespace, newline, whitespace},
};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let (nums, ops) = parse()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow!("Failed to parse input {e:?}"))?;

    let result: u64 = nums
        .iter()
        .zip(ops.iter())
        .map(|(xs, op)| match op {
            Op::Add => xs.iter().copied().sum::<u64>(),
            Op::Mul => xs.iter().copied().product::<u64>(),
        })
        .sum();

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let lines: Vec<_> = input.lines().collect();
    let (last, rest) = lines.split_last().unwrap();
    let t = transpose_str(&rest.join("\n"));

    let nums: Vec<Vec<u64>> = t
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|chunk| chunk.iter().flat_map(|s| s.parse::<u64>()).collect())
        .collect();

    let ops: Vec<_> = ops()
        .parse(last)
        .into_result()
        .map_err(|e| anyhow!("Failed to parse ops {e:?}"))?;

    let result: u64 = nums
        .iter()
        .zip(ops.iter())
        .map(|(xs, op)| match op {
            Op::Add => xs.iter().copied().sum::<u64>(),
            Op::Mul => xs.iter().copied().product::<u64>(),
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

fn transpose_str(s: &str) -> String {
    let lines: Vec<Vec<_>> = s.lines().map(|l| l.chars().collect()).collect();

    (0..lines[0].len())
        .map(|i| lines.iter().map(|l| l[i]).collect::<String>())
        .join("\n")
}

fn op<'src>() -> impl Parser<'src, &'src str, Op> {
    let mul = just('*').to(Op::Mul);
    let add = just('+').to(Op::Add);
    mul.or(add)
}

fn ops<'src>() -> impl Parser<'src, &'src str, Vec<Op>> {
    op().separated_by(whitespace())
        .collect()
        .then_ignore(whitespace())
}

fn nums<'src>() -> impl Parser<'src, &'src str, Vec<Vec<u64>>> {
    let num = text::int(10).map(|s: &str| s.parse::<u64>().unwrap());

    let nums = num
        .padded_by(inline_whitespace())
        .repeated()
        .at_least(1)
        .collect();

    nums.separated_by(newline()).collect().map(transpose)
}

fn parse<'src>() -> impl Parser<'src, &'src str, (Vec<Vec<u64>>, Vec<Op>)> {
    nums().then_ignore(newline()).then(ops())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    #[test]
    fn parse_nums() {
        let test_str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314";
        let parsed = nums().parse(test_str).into_output();
        assert_eq!(
            parsed,
            Some(vec![
                vec![123, 45, 6],
                vec![328, 64, 98],
                vec![51, 387, 215],
                vec![64, 23, 314]
            ])
        );
    }

    #[test]
    fn parse_ops() {
        let test_str = "*   +   *   +  ";
        let parsed = ops().parse(test_str).into_output();
        assert_eq!(parsed, Some(vec![Op::Mul, Op::Add, Op::Mul, Op::Add]));
    }

    #[test]
    fn parse_test() {
        let input = include_str!("../../samples/06.txt");

        let parsed = parse().parse(input).into_output();
        assert_eq!(
            parsed,
            Some((
                vec![
                    vec![123, 45, 6],
                    vec![328, 64, 98],
                    vec![51, 387, 215],
                    vec![64, 23, 314]
                ],
                vec![Op::Mul, Op::Add, Op::Mul, Op::Add]
            ))
        );
    }
}
