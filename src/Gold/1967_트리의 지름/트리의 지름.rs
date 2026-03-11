use crate::graph::Edge;
use std::io;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut tree = vec![vec![]; n + 1];
    for line in lines {
        let mut iter = line.split_ascii_whitespace().map(|x| x.parse().unwrap());
        let from: usize = iter.next().unwrap();
        let to = iter.next().unwrap();
        let weight = iter.next().unwrap();

        tree[from].push(Edge::new(to, weight));
        tree[to].push(Edge::new(from, weight));
    }

    println!("{}", graph::diameter_of_tree(n, 1, &tree));
}

mod graph {
    #[derive(Clone)]
    pub struct Edge {
        node: usize,
        cost: usize,
    }

    impl Edge {
        pub fn new(node: usize, cost: usize) -> Edge {
            Edge { node, cost }
        }
    }

    pub fn diameter_of_tree(n: usize, start: usize, tree: &Vec<Vec<Edge>>) -> usize {
        let mut distance_from_start = vec![None; n + 1];
        distance_from_start[start] = Some(0);

        let tmp_max = dfs(start, &mut distance_from_start, tree);
        for i in distance_from_start.iter_mut() {
            *i = None;
        }
        distance_from_start[tmp_max] = Some(0);

        let max = dfs(tmp_max, &mut distance_from_start, tree);
        distance_from_start[max].unwrap()
    }

    fn dfs(node: usize, distances: &mut Vec<Option<usize>>, tree: &[Vec<Edge>]) -> usize {
        let mut max_index = node;
        for next in tree[node].iter() {
            if distances[next.node].is_none() {
                distances[next.node] = Some(distances[node].unwrap() + next.cost);
                let child_max = dfs(next.node, distances, tree);
                if distances[max_index] < distances[child_max] {
                    max_index = child_max;
                }
            }
        }

        max_index
    }
}
