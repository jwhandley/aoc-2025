#! /bin/sh

if [ -f "src/solutions/day$1.rs" ]; then
exit 0
fi

touch "src/solutions/day$1.rs"
echo "
pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    todo!()
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() -> anyhow::Result<()> {
        Ok(())
    }
}
" > "src/solutions/day$1.rs"

str="pub mod day$1;"
file="src/solutions/mod.rs"

if ! grep -qF "$str" "$file"; then
    echo "$str" >> "$file"
fi