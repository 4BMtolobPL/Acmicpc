use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    // v(vertex) - 정점의 개수, e(edge) - 간선의 개수
    let (v, _e): (usize, usize) = {
        let mut iter = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let start: usize = lines.next().unwrap().parse().unwrap();

    // Edge에 대해 Clone을 구현하지 않기 위해 직접 초기화
    let mut graph: Vec<Vec<Edge>> = Vec::with_capacity(v + 1);
    for _ in 0..=v {
        graph.push(Vec::new());
    }

    for line in lines {
        let mut iter = line.split_ascii_whitespace().map(|x| x.parse().unwrap());

        let from: usize = iter.next().unwrap();
        let to = iter.next().unwrap();
        let weight = iter.next().unwrap();

        graph[from].push(Edge { to, cost: weight });
    }

    let mut out = String::new();
    for i in dijkstra(v, start, &graph).iter().skip(1) {
        if *i == usize::MAX {
            writeln!(out, "INF").unwrap();
        } else {
            writeln!(out, "{i}").unwrap();
        }
    }

    print!("{out}");
}

struct Edge {
    to: usize,
    cost: usize,
}

/// v - 정점의 개수
fn dijkstra(v: usize, start: usize, graph: &[Vec<Edge>]) -> Vec<usize> {
    // 1 ~ v 까지의 거리, 0은 사용하지 않음
    let mut distances = vec![usize::MAX; v + 1];
    let mut heap = BinaryHeap::new();

    distances[start] = 0;
    heap.push(Reverse((0, start)));

    while let Some(Reverse((dist, now))) = heap.pop() {
        if distances[now] < dist {
            continue;
        }

        for edge in graph[now].iter() {
            let next = (dist + edge.cost, edge.to);
            if next.0 < distances[next.1] {
                heap.push(Reverse(next));
                distances[next.1] = next.0;
            }
        }
    }

    distances
}
