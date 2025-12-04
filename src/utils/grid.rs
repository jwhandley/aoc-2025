use std::ops::{Index, IndexMut};

use glam::IVec2;

const ALL_DIRS: [IVec2; 8] = [
    IVec2::X,
    IVec2::NEG_X,
    IVec2::Y,
    IVec2::NEG_Y,
    IVec2::ONE,
    IVec2::NEG_ONE,
    IVec2::new(-1, 1),
    IVec2::new(1, -1),
];

pub struct Grid<T> {
    data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Index<IVec2> for Grid<T> {
    type Output = T;

    fn index(&self, index: IVec2) -> &Self::Output {
        let idx = self.index(index).unwrap();
        &self.data[idx]
    }
}

impl<T> IndexMut<IVec2> for Grid<T> {
    fn index_mut(&mut self, index: IVec2) -> &mut Self::Output {
        let idx = self.index(index).unwrap();
        &mut self.data[idx]
    }
}

impl<T> Grid<T> {
    pub fn parse(input: &str, f: fn(char) -> T) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let data: Vec<_> = lines.iter().flat_map(|s| s.chars()).map(|c| f(c)).collect();

        Grid {
            data,
            width,
            height,
        }
    }

    fn index(&self, i: IVec2) -> Option<usize> {
        let row = i.x as i32;
        let col = i.y as i32;

        if col < 0 || col >= self.width as i32 || row < 0 || row >= self.height as i32 {
            None
        } else {
            Some(row as usize * self.height + col as usize)
        }
    }

    pub fn neighbors(&self, pos: IVec2) -> impl Iterator<Item = Option<&T>> {
        ALL_DIRS
            .into_iter()
            .map(move |p| pos + p)
            .map(|nbr| self.get(nbr))
    }

    pub fn neighbor_indices(&self, pos: IVec2) -> impl Iterator<Item = IVec2> {
        ALL_DIRS
            .into_iter()
            .map(move |p| pos + p)
            .filter(|&nbr| self.index(nbr).is_some())
    }

    pub fn get(&self, index: IVec2) -> Option<&T> {
        let idx = self.index(index)?;
        self.data.get(idx)
    }

    pub fn indices(&self) -> impl Iterator<Item = IVec2> {
        (0..self.height).flat_map(|r| (0..self.width).map(move |c| IVec2::new(r as i32, c as i32)))
    }
}
