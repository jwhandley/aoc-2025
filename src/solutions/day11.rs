use std::collections::HashMap;

pub fn part1(input: &str) -> Result<String, anyhow::Error> {
    let mut id_map = HashMap::new();
    let mut graph = Graph::new();
    for (source, rest) in input.lines().map(|l| l.split_once(": ").unwrap()) {
        let from = *id_map.entry(source).or_insert_with(|| graph.add_node());

        for nbr in rest.split_whitespace() {
            let to = *id_map.entry(nbr).or_insert_with(|| graph.add_node());
            graph.add_edge(from, to);
        }
    }

    let you = id_map["you"];
    let out = id_map["out"];
    let total = graph.count_paths(you, out);

    Ok(total.to_string())
}

pub fn part2(input: &str) -> Result<String, anyhow::Error> {
    let mut id_map = HashMap::new();
    let mut graph = Graph::new();
    for (source, rest) in input.lines().map(|l| l.split_once(": ").unwrap()) {
        let from = *id_map.entry(source).or_insert_with(|| graph.add_node());

        for nbr in rest.split_whitespace() {
            let to = *id_map.entry(nbr).or_insert_with(|| graph.add_node());
            graph.add_edge(from, to);
        }
    }

    let svr = id_map["svr"];
    let out = id_map["out"];
    let dac = id_map["dac"];
    let fft = id_map["fft"];

    let svr_to_dac = graph.count_paths(svr, dac);
    let svr_to_fft = graph.count_paths(svr, fft);

    let fft_to_dac = graph.count_paths(fft, dac);
    let dac_to_fft = graph.count_paths(dac, fft);

    let fft_to_out = graph.count_paths(fft, out);
    let dac_to_out = graph.count_paths(dac, out);

    let svr_dac_fft_out = svr_to_dac * dac_to_fft * fft_to_out;
    let svr_fft_dac_out = svr_to_fft * fft_to_dac * dac_to_out;

    let total = svr_dac_fft_out + svr_fft_dac_out;

    Ok(total.to_string())
}

struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Self { adj: vec![] }
    }

    fn add_node(&mut self) -> usize {
        let id = self.adj.len();
        self.adj.push(vec![]);
        id
    }

    fn add_edge(&mut self, source: usize, target: usize) {
        self.adj[source].push(target);
    }

    fn neighbors(&self, node: usize) -> impl Iterator<Item = usize> {
        self.adj[node].iter().copied()
    }

    fn count_paths_memo(&self, from: usize, to: usize, memo: &mut Vec<usize>) -> usize {
        if from == to {
            return 1;
        }

        if memo[from] != usize::MAX {
            return memo[from];
        }

        let mut result = 0;
        for nbr in self.neighbors(from) {
            result += self.count_paths_memo(nbr, to, memo);
        }
        memo[from] = result;

        result
    }

    fn count_paths(&self, from: usize, to: usize) -> usize {
        let mut memo = vec![usize::MAX; self.adj.len()];
        self.count_paths_memo(from, to, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &'static str = include_str!("../../samples/11.1.txt");
    const INPUT_2: &'static str = include_str!("../../samples/11.2.txt");

    #[test]
    fn part1_test() -> anyhow::Result<()> {
        assert_eq!(part1(INPUT_1)?, "5".to_string());
        Ok(())
    }

    #[test]
    fn part2_test() -> anyhow::Result<()> {
        assert_eq!(part2(INPUT_2)?, "2".to_string());
        Ok(())
    }
}
