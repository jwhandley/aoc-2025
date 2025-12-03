use anyhow::Context;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let nums = parse_input(input);

    let (_, count) = nums.fold((50, 0), |(pos, count), amt| {
        let next = (pos + amt).rem_euclid(100);
        (next, count + if next == 0 { 1 } else { 0 })
    });

    Ok(count.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let nums = parse_input(input);

    let (_, count) = nums.fold((50, 0), |(pos, count), amt| {
        let total = pos + amt;
        let revolutions = (pos + amt).abs() / 100;

        (
            total.rem_euclid(100),
            count + revolutions + if pos != 0 && total <= 0 { 1 } else { 0 },
        )
    });

    Ok(count.to_string())
}

fn parse_input(input: &str) -> impl Iterator<Item = i32> {
    input.lines().flat_map(|l| {
        if let Some(rest) = l.strip_prefix('R') {
            rest.parse::<i32>().context("Failed to parse int")
        } else if let Some(rest) = l.strip_prefix('L') {
            rest.parse::<i32>()
                .context("Failed to parse int")
                .map(|n| -n)
        } else {
            anyhow::bail!("Should never reach this")
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
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

        if let Ok(r) = part1(input) {
            assert_eq!(r, "3".to_string());
        }
    }

    #[test]
    fn part2_test() {
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

        if let Ok(r) = part2(input) {
            assert_eq!(r, "6".to_string());
        }
    }
}
