use chumsky::{prelude::*, text::newline};
use glam::I64Vec3;
use itertools::Itertools;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    solve1(input, 1000).map(|n| n.to_string())
}

fn solve1(input: &str, n: usize) -> Result<usize, anyhow::Error> {
    let boxes = parser()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow::anyhow!("Unable to parse input {e:?}"))?;

    let mut uf = UnionFind::with_size(boxes.len());

    boxes
        .iter()
        .enumerate()
        .tuple_combinations()
        .sorted_by_key(|&((_, a), (_, b))| a.distance_squared(*b))
        .take(n)
        .for_each(|((i, _), (j, _))| {
            uf.union(i, j);
        });

    let result = uf
        .sizes
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .product::<usize>();

    Ok(result)
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let boxes = parser()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow::anyhow!("Unable to parse input {e:?}"))?;

    let mut uf = UnionFind::with_size(boxes.len());

    let result = boxes
        .iter()
        .enumerate()
        .tuple_combinations()
        .sorted_by_key(|&((_, a), (_, b))| a.distance_squared(*b))
        .find(|&((i, _), (j, _))| {
            uf.union(i, j);
            uf.is_connected()
        })
        .map(|((_, a), (_, b))| a.x * b.x)
        .unwrap();

    Ok(result.to_string())
}

struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    fn with_size(size: usize) -> Self {
        Self {
            parents: (0..size).collect(),
            sizes: vec![1; size],
            size,
        }
    }

    fn is_connected(&self) -> bool {
        self.sizes.contains(&self.size)
    }

    fn find(&mut self, i: usize) -> usize {
        let root = self.parents[i];

        if self.parents[root] != root {
            self.parents[i] = self.find(root);
            self.parents[i]
        } else {
            root
        }
    }

    fn union(&mut self, i: usize, j: usize) {
        let i_rep = self.find(i);
        let j_rep = self.find(j);

        if i_rep == j_rep {
            return;
        }

        let i_size = self.sizes[i_rep];
        let j_size = self.sizes[j_rep];

        if i_size < j_size {
            self.parents[i_rep] = j_rep;
            self.sizes[j_rep] += self.sizes[i_rep];
        } else {
            self.parents[j_rep] = i_rep;
            self.sizes[i_rep] += self.sizes[j_rep];
        }
    }
}

fn parser<'src>() -> impl Parser<'src, &'src str, Vec<I64Vec3>> {
    let num = text::int(10).map(|s: &str| s.parse::<i64>().unwrap());
    let vec3 = num
        .then_ignore(just(','))
        .then(num)
        .then_ignore(just(','))
        .then(num)
        .map(|((x, y), z)| I64Vec3::new(x, y, z));

    vec3.separated_by(newline()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() -> anyhow::Result<()> {
        let input = include_str!("../../samples/08.txt");
        assert_eq!(solve1(input, 10)?, 40);
        Ok(())
    }

    #[test]
    fn part2_test() -> anyhow::Result<()> {
        let input = include_str!("../../samples/08.txt");
        assert_eq!(part2(input)?, "25272".to_string());
        Ok(())
    }
}
