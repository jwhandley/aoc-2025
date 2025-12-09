use chumsky::{prelude::*, text::newline};
use glam::I64Vec2;
use itertools::Itertools;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let squares = squares()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow::anyhow!("Failed to parse squares: {e:?}"))?;

    let result = squares
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .unwrap();

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let squares = squares()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow::anyhow!("Failed to parse squares: {e:?}"))?;

    let lines: Vec<(I64Vec2, I64Vec2)> = squares.iter().copied().circular_tuple_windows().collect();

    let result = squares
        .into_iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            lines.iter().all(|(start, end)| {
                let left = a.x.max(b.x) <= start.x.min(end.x);
                let right = a.x.min(b.x) >= start.x.max(end.x);
                let below = a.y.max(b.y) <= start.y.min(end.y);
                let above = a.y.min(b.y) >= start.y.max(end.y);

                left || right || below || above
            })
        })
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .unwrap();

    Ok(result.to_string())
}

fn squares<'src>() -> impl Parser<'src, &'src str, Vec<I64Vec2>> {
    text::int(10)
        .then_ignore(just(','))
        .then(text::int(10))
        .map(|(a, b): (&str, &str)| {
            let x = a.parse().unwrap();
            let y = b.parse().unwrap();

            I64Vec2::new(x, y)
        })
        .separated_by(newline())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn part1_test() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT)?, "50".to_string());

        Ok(())
    }

    #[test]
    fn part2_test() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT)?, "24".to_string());

        Ok(())
    }
}
