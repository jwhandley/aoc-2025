use std::ops::{Index, IndexMut};

use glam::IVec2;

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
        let data: Vec<_> = lines.iter().flat_map(|s| s.chars()).map(|c| f(c)).collect();

        let height = lines.len();
        let width = data.len() / height;

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

    pub fn neighbors(&self, pos: IVec2, diag: bool) -> impl Iterator<Item = IVec2> {
        if diag {
            vec![
                pos + IVec2::X,
                pos - IVec2::X,
                pos + IVec2::Y,
                pos - IVec2::Y,
                pos + IVec2::ONE,
                pos - IVec2::ONE,
                pos + IVec2::new(-1, 1),
                pos + IVec2::new(1, -1),
            ]
        } else {
            vec![
                pos + IVec2::X,
                pos - IVec2::X,
                pos + IVec2::Y,
                pos - IVec2::Y,
            ]
        }
        .into_iter()
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
