use glam::IVec2;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let (start, height, splitters) = parse_input(input);

    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back(start);
    let mut total = 0;

    while let Some(next) = q.pop_front() {
        if next.x >= height as i32 || !seen.insert(next) {
            continue;
        }

        if splitters.contains(&(next + IVec2::X)) {
            total += 1;
            q.push_back(next + IVec2::new(1, -1));
            q.push_back(next + IVec2::new(1, 1));
        } else {
            q.push_back(next + IVec2::X);
        }
    }

    Ok(total.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let (start, height, splitters) = parse_input(input);

    let mut cache = HashMap::new();
    let total = solve(start, &splitters, height, &mut cache);
    Ok(total.to_string())
}

fn parse_input(input: &str) -> (IVec2, i32, HashSet<IVec2>) {
    let mut height = 0;
    let mut start = IVec2::ZERO;
    let mut splitters = HashSet::new();
    for (r, line) in input.lines().enumerate() {
        for (c, val) in line.char_indices() {
            if val == 'S' {
                start = IVec2::new(r as i32, c as i32);
            }

            if val == '^' {
                splitters.insert(IVec2::new(r as i32, c as i32));
            }
        }
        height += 1;
    }

    (start, height, splitters)
}

fn solve(
    pos: IVec2,
    splitters: &HashSet<IVec2>,
    height: i32,
    cache: &mut HashMap<IVec2, usize>,
) -> usize {
    if let Some(&n) = cache.get(&pos) {
        return n;
    }

    if pos.x >= height {
        return 1;
    }

    let result = if splitters.contains(&pos) {
        let left = solve(pos + IVec2::new(1, -1), splitters, height, cache);
        let right = solve(pos + IVec2::new(1, 1), splitters, height, cache);

        left + right
    } else {
        solve(pos + IVec2::X, splitters, height, cache)
    };

    cache.insert(pos, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() -> anyhow::Result<()> {
        let input = include_str!("../../samples/07.txt");

        assert_eq!(part1(input)?, "21".to_string());
        Ok(())
    }

    #[test]
    fn part2_test() -> anyhow::Result<()> {
        let input = include_str!("../../samples/07.txt");

        assert_eq!(part2(input)?, "40".to_string());
        Ok(())
    }
}
