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
                    .neighbors(coord, true)
                    .filter(|&p| matches!(grid.get(p), Some(Cell::Paper)))
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

    let mut total: usize = 0;
    loop {
        let to_remove: Vec<_> = grid
            .indices()
            .filter(|&coord| match grid[coord] {
                Cell::Paper => {
                    let count = grid
                        .neighbors(coord, true)
                        .filter(|&p| matches!(grid.get(p), Some(Cell::Paper)))
                        .count();

                    count < 4
                }
                Cell::Empty => false,
            })
            .collect();

        if to_remove.is_empty() {
            break;
        }

        for coord in to_remove.iter() {
            grid[*coord] = Cell::Empty;
            total += 1;
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
