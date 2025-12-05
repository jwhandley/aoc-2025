use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
};

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let (_, nums) = directions.parse(input).unwrap();

    let (_, count) = nums.iter().fold((50, 0), |(pos, count), amt| {
        let next = (pos + amt).rem_euclid(100);
        (next, count + if next == 0 { 1 } else { 0 })
    });

    Ok(count.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let (_, nums) = directions.parse(input).unwrap();

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

fn directions(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(line_ending, direction).parse(input)
}

fn direction(input: &str) -> IResult<&str, i32> {
    let (input, dir) = alt((tag("L"), tag("R"))).parse(input)?;
    let (input, num) = complete::i32(input)?;

    let d = match dir {
        "L" => -num,
        "R" => num,
        x => panic!("Unknown {x}"),
    };

    Ok((input, d))
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
