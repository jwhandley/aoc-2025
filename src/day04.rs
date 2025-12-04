use std::collections::HashMap;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let mut grid = parse_input(input);
    let total = grid.remove_paper();

    Ok(total.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let mut grid = parse_input(input);

    let mut total = 0;
    loop {
        let removed = grid.remove_paper();
        if removed == 0 {
            break;
        }
        total += removed;
    }

    Ok(total.to_string())
}

enum Cell {
    Paper,
    Empty,
}

type Pos = (i32, i32);

fn neighbors(pos: Pos) -> impl Iterator<Item = Pos> {
    let (r, c) = pos;

    (-1..=1)
        .flat_map(move |dr| (-1..=1).map(move |dc| (r + dr, c + dc)))
        .filter(move |&nbr| nbr != pos)
}

struct Grid {
    cells: HashMap<Pos, Cell>,
}

impl Grid {
    fn remove_paper(&mut self) -> usize {
        let mut to_remove = vec![];
        for (coord, cell) in self.cells.iter() {
            let count = self.count_neighbors(*coord);

            if let Cell::Paper = cell
                && count < 4
            {
                to_remove.push(*coord);
            }
        }

        for coord in to_remove.iter() {
            self.cells.insert(*coord, Cell::Empty);
        }

        to_remove.len()
    }

    fn count_neighbors(&self, coord: Pos) -> usize {
        neighbors(coord)
            .filter(|nbr| match self.cells.get(nbr) {
                Some(Cell::Paper) => true,
                _ => false,
            })
            .count()
    }
}

fn parse_input(input: &str) -> Grid {
    let input = input.as_bytes();
    let mut cells = HashMap::new();

    for (r, line) in input.split(|&c| c == b'\n').enumerate() {
        for (c, cell) in line.iter().enumerate() {
            let v = match cell {
                b'@' => Cell::Paper,
                _ => Cell::Empty,
            };
            cells.insert((r as i32, c as i32), v);
        }
    }

    Grid { cells }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../samples/04.txt");

        if let Ok(result) = part1(input) {
            assert_eq!(result, "13".to_string());
        }
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../samples/04.txt");

        if let Ok(result) = part2(input) {
            assert_eq!(result, "43".to_string());
        }
    }
}
