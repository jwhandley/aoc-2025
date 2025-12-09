use chumsky::{prelude::*, text::newline};
use geo::{Contains, Coord, LineString, Polygon, Rect, coord};
use itertools::Itertools;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let squares = squares()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow::anyhow!("Failed to parse squares: {e:?}"))?;

    let result = squares
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let diff = a - b;
            (diff.x.abs() + 1.) * (diff.y.abs() + 1.)
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    Ok(result.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let squares = squares()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow::anyhow!("Failed to parse squares: {e:?}"))?;

    let polygon = Polygon::new(LineString::new(squares.clone()), vec![]);

    let result = squares
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            let rect = Rect::new(*a, *b);

            if polygon.contains(&rect) {
                Some(rect)
            } else {
                None
            }
        })
        .map(|r| (r.width() + 1.) * (r.height() + 1.))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    Ok(result.to_string())
}

fn squares<'src>() -> impl Parser<'src, &'src str, Vec<Coord>> {
    text::int(10)
        .then_ignore(just(','))
        .then(text::int(10))
        .map(|(a, b): (&str, &str)| {
            let x = a.parse().unwrap();
            let y = b.parse().unwrap();

            coord! {x: x, y: y}
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
