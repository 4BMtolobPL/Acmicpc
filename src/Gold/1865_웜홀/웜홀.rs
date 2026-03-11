use std::fmt::Write;
use std::io;
use std::str::Lines;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();

    let test_cases = lines.next().unwrap().parse().unwrap();

    let get_line_parse = |lines: &mut Lines| {
        let mut iter = lines.next().unwrap().split_ascii_whitespace();
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };

    let mut out = String::new();
    for _ in 0..test_cases {
        let (n, m, w): (usize, _, _) = get_line_parse(&mut lines);

        let mut graph = vec![];
        for _ in 0..m {
            let (s, e, t): (usize, usize, i32) = get_line_parse(&mut lines);

            graph.push(Edge::new(s - 1, e - 1, t));
            graph.push(Edge::new(e - 1, s - 1, t));
        }

        for _ in 0..w {
            let (s, e, t): (usize, usize, i32) = get_line_parse(&mut lines);

            graph.push(Edge::new(s - 1, e - 1, -t));
        }

        if has_negative_cycle(n, &graph) {
            writeln!(out, "YES").unwrap();
        } else {
            writeln!(out, "NO").unwrap();
        }
    }

    print!("{out}");
}

fn has_negative_cycle(n: usize, edges: &[Edge]) -> bool {
    let mut distances = vec![0; n];

    for _ in 0..(n - 1) {
        let mut is_changed = false;
        for &Edge { from, to, cost } in edges.iter() {
            if distances[from] + cost < distances[to] {
                distances[to] = distances[from] + cost;
                is_changed = true;
            }
        }

        if !is_changed {
            return false;
        }
    }

    // 음수 사이클 확인
    for &Edge { from, to, cost } in edges.iter() {
        if distances[from] + cost < distances[to] {
            return true;
        }
    }

    false
}

#[derive(Clone)]
pub struct Edge {
    from: usize,
    to: usize,
    cost: i32,
}

impl Edge {
    pub fn new(from: usize, to: usize, cost: i32) -> Edge {
        Edge { from, to, cost }
    }
}
