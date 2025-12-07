pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let lines: Vec<&str> = input.lines().collect();
    let (first, rest) = lines.split_first().unwrap();

    let mut cols: Vec<bool> = first.chars().map(|c| c == 'S').collect();
    let mut count = 0;
    for row in rest.iter() {
        for (i, c) in row.char_indices() {
            if cols[i] && c == '^' {
                cols[i - 1] = true;
                cols[i] = false;
                cols[i + 1] = true;
                count += 1;
            }
        }
    }

    Ok(count.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let lines: Vec<&str> = input.lines().collect();
    let (first, rest) = lines.split_first().unwrap();

    let mut cols: Vec<usize> = first
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect();

    for row in rest.iter() {
        for (i, c) in row.char_indices() {
            if cols[i] > 0 && c == '^' {
                cols[i - 1] += cols[i];
                cols[i + 1] += cols[i];
                cols[i] = 0;
            }
        }
    }

    let total: usize = cols.iter().sum();
    Ok(total.to_string())
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
