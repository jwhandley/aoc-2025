use anyhow::anyhow;
use chumsky::prelude::*;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let nums = directions()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow!("Failed to parse input {e:?}"))?;

    let (_, count) = nums.iter().fold((50, 0), |(pos, count), amt| {
        let next = (pos + amt).rem_euclid(100);
        (next, count + if next == 0 { 1 } else { 0 })
    });

    Ok(count.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let nums = directions()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow!("Failed to parse input {e:?}"))?;

    let (_, count) = nums.iter().fold((50, 0), |(pos, count), amt| {
        let total = pos + amt;
        let revolutions = (pos + amt).abs() / 100;

        (
            total.rem_euclid(100),
            count + revolutions + if pos != 0 && total <= 0 { 1 } else { 0 },
        )
    });

    Ok(count.to_string())
}

fn directions<'src>() -> impl Parser<'src, &'src str, Vec<i32>> {
    let int32 = text::int(10).map(|v: &str| v.parse::<i32>().unwrap());
    let left = just('L').ignore_then(int32).map(|v| -v);
    let right = just('R').ignore_then(int32);

    left.or(right).separated_by(text::newline()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() -> anyhow::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part1(input)?, "3".to_string());
        Ok(())
    }

    #[test]
    fn part2_test() -> anyhow::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part2(input)?, "6".to_string());
        Ok(())
    }
}
