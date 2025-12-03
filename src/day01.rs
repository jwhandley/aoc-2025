pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let nums = input.lines().flat_map(|l| {
        if l.starts_with('R') {
            l[1..].parse::<i32>()
        } else {
            l[1..].parse::<i32>().map(|n| n * -1)
        }
    });

    let (_, count) = nums.fold((50, 0), |(pos, count), amt| {
        let next = (pos + amt).rem_euclid(100);
        (next, count + if next == 0 { 1 } else { 0 })
    });

    Ok(count.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let nums = input.lines().flat_map(|l| {
        if l.starts_with('R') {
            l[1..].parse::<i32>()
        } else {
            l[1..].parse::<i32>().map(|n| n * -1)
        }
    });

    let (_, count) = nums.fold((50, 0), |(pos, count), amt| {
        let next = (pos + amt).rem_euclid(100);
        let revolutions = (pos + amt).abs() / 100;

        (
            next,
            count + revolutions + if pos != 0 && (pos + amt) <= 0 { 1 } else { 0 },
        )
    });

    Ok(count.to_string())
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
