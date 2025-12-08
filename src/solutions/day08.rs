use chumsky::{prelude::*, text::newline};
use glam::I64Vec3;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    solve1(input, 1000).map(|n| n.to_string())
}

fn solve1(input: &str, n: usize) -> Result<usize, anyhow::Error> {
    let boxes = parser()
        .parse(input)
        .into_result()
        .map_err(|e| anyhow::anyhow!("Unable to parse input {e:?}"))?;

    let edges: Vec<_> = boxes
        .iter()
        .copied()
        .tuple_combinations()
        .sorted_by_key(|&(a, b)| a.distance_squared(b))
        .take(n)
        .collect();

    let g = Graph::from_slice(&edges);

    let result = g
        .connected_components()
        .iter()
        .map(|g| g.len())
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

    let edges: Vec<_> = boxes
        .iter()
        .copied()
        .tuple_combinations()
        .sorted_by_key(|&(a, b)| a.distance_squared(b))
        .collect();

    let mut circuits: Vec<HashSet<I64Vec3>> = boxes.iter().map(|&b| HashSet::from([b])).collect();

    for &(a, b) in edges.iter() {
        let (inside, mut outside): (Vec<_>, Vec<_>) = circuits
            .into_iter()
            .partition(|c| c.contains(&a) || c.contains(&b));

        if outside.is_empty() {
            return Ok((a.x * b.x).to_string());
        }

        let new_circuit: HashSet<I64Vec3> =
            inside.iter().fold(HashSet::from([a, b]), |acc, next| {
                acc.union(next).copied().collect()
            });

        outside.push(new_circuit);
        circuits = outside;
    }

    anyhow::bail!("Couldn't find answer")
}

type NodeId = usize;
#[derive(Default, Debug)]
struct Graph {
    adj: Vec<HashSet<NodeId>>,
}

impl Graph {
    fn add_node(&mut self) -> NodeId {
        let id = self.adj.len();
        self.adj.push(HashSet::new());
        id
    }

    fn add_edge(&mut self, from: NodeId, to: NodeId) {
        self.adj[from].insert(to);
        self.adj[to].insert(from);
    }

    fn from_slice<N: Eq + Hash>(edges: &[(N, N)]) -> Self {
        let mut g = Self::default();

        let mut id_map = HashMap::new();
        for (from, to) in edges.iter() {
            let from_id = *id_map.entry(from).or_insert_with(|| g.add_node());
            let to_id = *id_map.entry(to).or_insert_with(|| g.add_node());
            g.add_edge(from_id, to_id);
        }

        g
    }

    fn connected_components(&self) -> Vec<Vec<NodeId>> {
        let mut map: HashMap<NodeId, usize> = HashMap::new();
        let mut current = 0;
        for start_node in 0..self.adj.len() {
            if map.contains_key(&start_node) {
                continue;
            }

            let mut q = VecDeque::new();
            q.push_back(start_node);

            while let Some(node) = q.pop_front() {
                if map.contains_key(&node) {
                    continue;
                }

                map.insert(node, current);

                for &nbr in self.adj[node].iter() {
                    q.push_back(nbr);
                }
            }
            current += 1;
        }

        map.into_iter()
            .into_group_map_by(|(_, component)| *component)
            .values()
            .map(|group| group.iter().map(|&(node, _)| node).collect())
            .collect()
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
