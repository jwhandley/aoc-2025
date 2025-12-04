use std::collections::HashSet;

use crate::utils::grid::Grid;
use glam::IVec2;

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
    let mut total: usize = 0;
    while let Some(pos) = to_remove.pop() {
        if !removed.insert(pos) {
            continue;
        }

        grid[pos] = Cell::Empty;
        total += 1;

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

    Ok(total.to_string())
}

#[derive(Debug)]
enum Cell {
    Paper,
    Empty,
}

impl std::fmt::Display for Grid<Cell> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                let idx = IVec2::new(r as i32, c as i32);
                match &self[idx] {
                    Cell::Paper => write!(f, "@"),
                    Cell::Empty => write!(f, "."),
                }?
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../../samples/04.txt");

        if let Ok(result) = part1(input) {
            assert_eq!(result, "13".to_string());
        }
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../../samples/04.txt");

        if let Ok(result) = part2(input) {
            assert_eq!(result, "43".to_string());
        }
    }
}
