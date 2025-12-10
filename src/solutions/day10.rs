use good_lp::{Expression, ProblemVariables, Solution, SolverModel, constraint, variable};

use std::collections::{HashSet, VecDeque};

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let total: usize = input
        .lines()
        .map(|line| {
            let (pattern, buttons) = parse1(line);
            solve(pattern, &buttons)
        })
        .sum();

    Ok(total.to_string())
}

fn solve(target: usize, options: &[usize]) -> usize {
    let mut q = VecDeque::new();
    q.push_back((target, 0));

    let mut seen = HashSet::new();

    while let Some((current, n)) = q.pop_front() {
        if current == 0 {
            return n;
        }

        if !seen.insert((current, n)) {
            continue;
        }

        for option in options {
            q.push_back((current ^ option, n + 1));
        }
    }

    usize::MAX
}

fn solve2(line: &str) -> Result<usize, good_lp::ResolutionError> {
    let parts = line.split_whitespace().skip(1);
    let (buttons, target): (Vec<_>, Vec<_>) = parts.partition(|p| p.starts_with('('));

    let target: Vec<usize> = target[0]
        .trim_matches(|c| c == '{' || c == '}')
        .split(',')
        .flat_map(|s| s.parse())
        .collect();

    let n = target.len();

    let buttons: Vec<Vec<usize>> = buttons
        .iter()
        .map(|b| {
            let idx = b
                .trim_matches(|c| c == '(' || c == ')')
                .split(',')
                .flat_map(|s| s.parse::<usize>());

            let mut v = vec![0; n];
            for i in idx {
                v[i] = 1;
            }
            v
        })
        .collect();

    let mut problem = ProblemVariables::new();
    let counts = problem.add_vector(variable().integer().min(0), buttons.len());
    let total: Expression = counts.iter().sum();
    let mut model = problem.minimise(total).using(good_lp::default_solver);
    for i in 0..target.len() {
        let target_i = target[i] as f64;
        let expr_i: Expression = buttons
            .iter()
            .map(|b| b[i])
            .enumerate()
            .map(|(j, v)| counts[j] * v as f64)
            .sum();

        model.add_constraint(constraint!(expr_i == target_i));
    }

    let solution = model.solve();
    solution.map(|s| counts.iter().map(|c| s.value(*c).round()).sum::<f64>() as usize)
}

fn parse1(line: &str) -> (usize, Vec<usize>) {
    let mut parts = line.split_whitespace();
    let (pattern, size) = parts
        .next()
        .map(|s| {
            let pattern = s.trim_matches(|c| c == '[' || c == ']');
            let n = pattern
                .chars()
                .fold(0, |acc, next| acc * 2 + if next == '#' { 1 } else { 0 });
            let size = pattern.len();
            (n, size)
        })
        .unwrap();

    let buttons = parts
        .take_while(|p| p.starts_with('('))
        .map(|s| {
            let button = s.trim_matches(|c| c == '(' || c == ')');
            let mut n: usize = 0;
            for s in button.split(',') {
                let i = s.parse::<usize>().unwrap();
                n |= 1 << (size - i - 1);
            }

            n
        })
        .collect();

    (pattern, buttons)
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let mut total = 0;
    for result in input.lines().map(solve2) {
        total += result.expect("Should have been able to solve");
    }

    Ok(total.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../../samples/10.txt");

    #[test]
    fn part1_test() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, "7".to_string());

        Ok(())
    }

    #[test]
    fn part2_test() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT)?, "33".to_string());

        Ok(())
    }
}
