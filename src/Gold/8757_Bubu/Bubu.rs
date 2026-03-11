use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();
    let (n, m, s): (usize, _, _) = {
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

    let mut graph = vec![vec![]; n + 1];
    for line in lines.by_ref().take(m) {
        let mut iter = line.split_ascii_whitespace().map(|x| x.parse().unwrap());

        let a: usize = iter.next().unwrap();
        let b = iter.next().unwrap();
        let w = iter.next().unwrap();

        graph[a].push(Edge::new(b, w));
        graph[b].push(Edge::new(a, w));
    }

    let mut patrol_positions: Vec<usize> = vec![];
    for line in lines.by_ref().take(s) {
        patrol_positions.push(line.parse().unwrap());
    }

    let bubu_position: usize = lines.next().unwrap().parse().unwrap();

    let patrol_distances = multi_shortest(&patrol_positions, &graph);
    if let Some(shortest) = dijkstra_avoid_patrol(bubu_position, 1, &patrol_distances, &graph) {
        println!("{shortest}");
    } else {
        println!("-1");
    }
}

#[derive(Copy, Clone)]
struct Edge {
    next: usize,
    cost: usize,
}

impl Edge {
    fn new(next: usize, cost: usize) -> Edge {
        Edge { next, cost }
    }
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

fn multi_shortest(starts: &[usize], graph: &[Vec<Edge>]) -> Vec<usize> {
    let mut distances: Vec<usize> = vec![usize::MAX; graph.len()];

    for start in starts.iter() {
        dijkstra_all(*start, &mut distances, graph);
    }

    distances
}

fn dijkstra_all(from: usize, distances: &mut [usize], graph: &[Vec<Edge>]) {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    distances[from] = 0;
    heap.push(State {
        cost: 0,
        position: from,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > distances[position] {
            continue;
        }

        for edge in graph[position].iter() {
            let next = State {
                cost: cost + edge.cost,
                position: edge.next,
            };

            if next.cost < distances[next.position] {
                heap.push(next);
                distances[next.position] = next.cost;
            }
        }
    }
}

fn dijkstra_avoid_patrol(
    from: usize,
    to: usize,
    patrol_distances: &[usize],
    graph: &[Vec<Edge>],
) -> Option<usize> {
    // 시작지점부터 경비와 같이 있다면
    if patrol_distances[from] == 0 {
        return None;
    }

    let mut distances: Vec<usize> = vec![usize::MAX; graph.len()];
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    distances[from] = 0;
    heap.push(State {
        cost: 0,
        position: from,
    });
    while let Some(State { cost, position }) = heap.pop() {
        if position == to {
            return Some(cost);
        }

        if cost > distances[position] {
            continue;
        }
        for edge in graph[position].iter() {
            let next = State {
                cost: cost + edge.cost,
                position: edge.next,
            };
            if next.cost < distances[next.position] && next.cost < patrol_distances[next.position] {
                heap.push(next);
                distances[next.position] = next.cost;
            }
        }
    }

    None
}
