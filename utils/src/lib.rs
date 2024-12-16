use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    fs::File,
    hash::Hash,
    io::{BufRead, BufReader, Lines},
    str::FromStr,
};

use num::Integer;

fn open_file(file_path: &str) -> File {
    File::open(file_path).unwrap()
}

pub struct AocBufReader {
    iter: Lines<BufReader<File>>,
}

impl AocBufReader {
    fn from_file(file_handle: File) -> AocBufReader {
        AocBufReader {
            iter: BufReader::new(file_handle).lines(),
        }
    }

    pub fn from_string(file_path: &str) -> AocBufReader {
        AocBufReader::from_file(open_file(file_path))
    }
}

impl Iterator for AocBufReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(result) => match result {
                Ok(line) => Some(line),
                Err(error) => panic!("{}", error),
            },
            None => None,
        }
    }
}

pub fn parse_iter<T: FromStr + Debug, U: AsRef<str>>(
    input: impl Iterator<Item = U>,
) -> impl Iterator<Item = T>
where
    <T as FromStr>::Err: Debug,
{
    input.map(|x| x.as_ref().parse::<T>().unwrap())
}

pub trait DijkstraSearchable {
    type Node;
    type Cost;

    fn neighbors(
        &self,
        previous: &Self::Node,
        previous_cost: Self::Cost,
    ) -> Vec<(Self::Node, Self::Cost)>;
}

pub fn shortest_path_length<
    N: Eq + PartialEq + Hash + Clone,
    C: Integer + Copy,
    G: DijkstraSearchable<Node = N, Cost = C>,
>(
    graph: G,
    start: N,
    ends: HashSet<N>,
) -> Option<C> {
    let mut cost_to_reach: HashMap<N, C> = HashMap::from([(start, C::zero())]);
    let mut visited: HashSet<N> = HashSet::new();

    while let Some((next, cost)) = cost_to_reach
        .iter()
        .filter(|(node, _)| !visited.contains(node))
        .min_by_key(|(_, cost)| *cost)
    {
        if ends.contains(next) {
            return Some(*cost);
        }

        visited.insert(next.clone());
        for (neighbor, neighbor_cost) in graph.neighbors(next, *cost) {
            let updated_cost = match cost_to_reach.get(&neighbor) {
                Some(old_cost) => std::cmp::min(old_cost, &neighbor_cost),
                None => &neighbor_cost,
            };
            cost_to_reach.insert(neighbor, *updated_cost);
        }
    }

    None
}

/// Find the length of the shortest paths from start to any end
/// in the collection of ends. Return this shortest length as well
/// as all unique paths from start to an end (sequences of nodes)
/// that have this length
pub fn shortest_paths<
    N: Eq + PartialEq + Hash + Clone + Debug,
    C: Integer + Copy,
    G: DijkstraSearchable<Node = N, Cost = C>,
>(
    graph: G,
    start: N,
    ends: HashSet<N>,
) -> Option<(C, Vec<Vec<N>>)> {
    // the cost to reach a given node and the nodes from which you
    // can reach it with that cost
    let mut cost_to_reach: HashMap<N, (C, HashSet<N>)> =
        HashMap::from([(start.clone(), (C::zero(), HashSet::new()))]);
    let mut visited: HashSet<N> = HashSet::new();

    let mut shortest_path_length: Option<C> = None;
    loop {
        let candidate = cost_to_reach
            .iter()
            .filter(|(node, (cost, _))| {
                !visited.contains(node)
                    && (shortest_path_length.is_none() || *cost <= shortest_path_length.unwrap())
            })
            .min_by_key(|(_, (cost, _))| *cost)
            .map(|(x, (c, p))| (x.clone(), (*c, p.clone())));

        if candidate.is_none() {
            break;
        }
        let (next, (cost, _)) = candidate.unwrap();

        visited.insert(next.clone());
        for (neighbor, neighbor_cost) in graph.neighbors(&next, cost) {
            if cost_to_reach.contains_key(&neighbor) {
                let (mut best_cost, mut previous) = cost_to_reach.remove(&neighbor).unwrap();
                match neighbor_cost.cmp(&best_cost) {
                    std::cmp::Ordering::Equal => {
                        previous.insert(next.clone());
                    }
                    std::cmp::Ordering::Less => {
                        best_cost = neighbor_cost;
                        previous = HashSet::from([next.clone()]);
                    }
                    _ => (),
                }
                cost_to_reach.insert(neighbor, (best_cost, previous));
            } else {
                if ends.contains(&neighbor)
                    && (shortest_path_length.is_none()
                        || neighbor_cost <= shortest_path_length.unwrap())
                {
                    shortest_path_length = Some(neighbor_cost);
                }
                cost_to_reach.insert(neighbor, (neighbor_cost, HashSet::from([next.clone()])));
            }
        }
    }

    shortest_path_length?;

    let reverse_map = cost_to_reach
        .into_iter()
        .map(|(from, (_, to))| (from, to))
        .collect();
    let mut paths: Vec<Vec<N>> = Vec::new();
    for end in ends {
        if let Some(paths_) = backpropagate(&start, end, &reverse_map) {
            paths.extend(paths_);
        }
    }

    Some((shortest_path_length.unwrap(), paths))
}

fn backpropagate<N: Eq + PartialEq + Hash + Clone + Debug>(
    start: &N,
    end: N,
    reverse_map: &HashMap<N, HashSet<N>>,
) -> Option<Vec<Vec<N>>> {
    if start == &end {
        return Some(vec![vec![start.clone()]]);
    }

    match reverse_map.get(&end) {
        Some(previous) => {
            let mut result: Vec<Vec<N>> = Vec::new();
            for p in previous.clone() {
                if let Some(mut the_rest) = backpropagate(start, p, reverse_map) {
                    for path in the_rest.iter_mut() {
                        path.push(end.clone());
                    }
                    result.extend(the_rest);
                }
            }
            Some(result)
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ints() {
        let buffer = AocBufReader::from_string("src/data/test_parse_ints.txt");
        assert_eq!(
            parse_iter::<usize, _>(buffer).collect::<Vec<_>>(),
            vec![1, 2, 3, 4]
        );
    }
}
