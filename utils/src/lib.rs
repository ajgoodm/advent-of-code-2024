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

pub fn shortest_path<
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
            *cost_to_reach.entry(neighbor).or_insert(C::zero()) = *updated_cost;
        }
    }

    None
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
