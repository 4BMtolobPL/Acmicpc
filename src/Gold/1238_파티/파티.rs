use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n: usize = iter.next().unwrap();
    let m = iter.next().unwrap();
    let x = iter.next().unwrap() - 1;

    let mut graph = vec![vec![]; n];
    let mut rev_graph = vec![vec![]; n];

    for _ in 0..m {
        let from = iter.next().unwrap() - 1;
        let to = iter.next().unwrap() - 1;
        let cost = iter.next().unwrap();

        graph[from].push(Edge::new(to, cost));
        rev_graph[to].push(Edge::new(from, cost));
    }

    let distances = dijkstra(n, x, &graph);
    let rev_distances = dijkstra(n, x, &rev_graph);

    println!(
        "{}",
        distances
            .iter()
            .zip(rev_distances.iter())
            .map(|(a, b)| a + b)
            .max()
            .unwrap()
    );
}

fn dijkstra(v: usize, start: usize, graph: &[Vec<Edge>]) -> Vec<usize> {
    let mut distances = vec![usize::MAX; v];
    let mut heap = BinaryHeap::new();

    distances[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if distances[position] < cost {
            continue;
        }

        for edge in graph[position].iter() {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };
            if next.cost < distances[next.position] {
                heap.push(next);
                distances[next.position] = next.cost;
            }
        }
    }

    distances
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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
