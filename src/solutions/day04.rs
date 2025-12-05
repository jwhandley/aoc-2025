use crate::utils::grid::Grid;
use std::collections::HashSet;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let grid = Grid::parse(input, |c| match c {
        '@' => Cell::Paper,
        _ => Cell::Empty,
    });

    let total = grid
        .indices()
        .filter(|&coord| match grid[coord] {
            Cell::Paper => {
                let count = grid
                    .neighbors(coord)
                    .filter(|cell: &Option<&Cell>| matches!(cell, Some(Cell::Paper)))
                    .count();

                count < 4
            }
            Cell::Empty => false,
        })
        .count();

    Ok(total.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let mut grid = Grid::parse(input, |c| match c {
        '@' => Cell::Paper,
        _ => Cell::Empty,
    });

    let mut to_remove: Vec<_> = grid
        .indices()
        .filter(|&pos| {
            let count = grid
                .neighbors(pos)
                .filter(|&cell| matches!(cell, Some(Cell::Paper)))
                .count();

            matches!(grid[pos], Cell::Paper) && count < 4
        })
        .collect();

    let mut removed = HashSet::new();
    while let Some(pos) = to_remove.pop() {
        if !removed.insert(pos) {
            continue;
        }

        grid[pos] = Cell::Empty;

        for nbr in grid.neighbor_indices(pos) {
            let count = grid
                .neighbors(nbr)
                .filter(|&cell| matches!(cell, Some(Cell::Paper)))
                .count();

            if count < 4 && matches!(grid[nbr], Cell::Paper) {
                to_remove.push(nbr);
            }
        }
    }

    Ok(removed.len().to_string())
}

#[derive(Debug)]
enum Cell {
    Paper,
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() -> anyhow::Result<()> {
        let input = include_str!("../../samples/04.txt");
        assert_eq!(part1(input)?, "13".to_string());
        Ok(())
    }

    #[test]
    fn part2_test() -> anyhow::Result<()> {
        let input = include_str!("../../samples/04.txt");

        assert_eq!(part2(input)?, "43".to_string());
        Ok(())
    }
}
