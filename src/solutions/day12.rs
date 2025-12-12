use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Tile {
    fill: [[bool; 3]; 3],
}

impl Tile {
    fn area(&self) -> usize {
        let mut total = 0;
        for r in 0..3 {
            for c in 0..3 {
                if self.fill[r][c] {
                    total += 1;
                }
            }
        }

        total
    }
}

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let parts: Vec<_> = input.split("\n\n").collect();
    let (grids, tiles) = parts.split_last().unwrap();
    let tiles: HashMap<usize, Tile> = tiles.into_iter().map(|s| parse_tile(s)).collect();
    let grids: Vec<_> = grids.lines().map(|s| parse_grid(s)).collect();

    let total = grids
        .iter()
        .filter(|((width, height), counts)| {
            let total_area = width * height;
            let needed_area: usize = counts
                .iter()
                .map(|(i, count)| tiles[i].area() * count)
                .sum();
            needed_area <= total_area
        })
        .count();

    Ok(total.to_string())
}

fn parse_grid(s: &str) -> ((usize, usize), HashMap<usize, usize>) {
    let (dim, counts) = s.split_once(": ").unwrap();

    let (width, length) = dim.split_once('x').unwrap();
    let width: usize = width.parse().unwrap();
    let length: usize = length.parse().unwrap();

    let counts: HashMap<usize, usize> = counts
        .split_whitespace()
        .enumerate()
        .map(|(i, s)| (i, s.parse().unwrap()))
        .collect();

    ((width, length), counts)
}

fn parse_tile(s: &str) -> (usize, Tile) {
    let (id, cells) = s.split_once('\n').unwrap();
    let id: usize = id.trim_end_matches(':').parse().unwrap();

    let mut fill = [[false; 3]; 3];

    for (r, row) in cells.lines().enumerate() {
        for (c, cell) in row.char_indices() {
            fill[r][c] = cell == '#';
        }
    }

    (id, Tile { fill })
}

pub fn part2(_input: &str) -> Result<String, anyhow::Error> {
    Ok("Done!".to_string())
}
