use std::collections::HashMap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();
    let (n, m, _k): (usize, _, _) = {
        let mut iter = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap());
        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    let mut graph: Vec<Vec<Edge>> = vec![vec![]; n + 1];

    for line in lines {
        let mut iter = line.split_ascii_whitespace().map(|x| x.parse().unwrap());
        let a: usize = iter.next().unwrap();
        let b = iter.next().unwrap();
        let c = iter.next().unwrap();

        if a < b {
            graph[a].push(Edge::new(b, c));
        }
    }

    let mut max_cost = vec![HashMap::new(); n + 1];
    max_cost[1].insert(1, 0);

    for i in 1..=n {
        let now = max_cost[i].clone();
        for (count, cost) in now.iter() {
            if *count >= m {
                continue;
            }

            for edge in graph[i].iter() {
                let new_cost = cost + edge.cost;
                let entry = max_cost[edge.to].entry(count + 1).or_insert(0);
                if new_cost > *entry {
                    *entry = new_cost;
                }
            }
        }
    }

    println!("{}", max_cost[n].values().max().unwrap());
}

#[derive(Clone)]
struct Edge {
    to: usize,
    cost: usize,
}

impl Edge {
    fn new(to: usize, cost: usize) -> Edge {
        Edge { to, cost }
    }
}
