use std::collections::HashSet;
use std::fmt::Write as fmtWrite;
use std::io::Write;
use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter, Read},
};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let mut iter = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap());
    let n: usize = iter.next().unwrap();
    iter.next();
    let v = iter.next().unwrap();

    let mut graph = vec![Vec::new(); n + 1];

    for line in lines {
        let mut nums = line.split_whitespace().map(|x| x.parse().unwrap());
        let lhs: usize = nums.next().unwrap();
        let rhs = nums.next().unwrap();
        graph[lhs].push(rhs);
        graph[rhs].push(lhs);
    }
    for item in &mut graph {
        item.sort_unstable();
    }

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);
    writeln!(out, "{}", dfs(v, &graph)).unwrap();
    writeln!(out, "{}", bfs(v, &graph)).unwrap();
}

fn dfs(start: usize, graph: &[Vec<usize>]) -> String {
    let mut stack = vec![start];
    let mut visited = HashSet::new();
    let mut out = String::new();
    while !stack.is_empty() {
        let last = stack.pop().unwrap();
        if visited.contains(&last) {
            continue;
        } else {
            for i in graph[last].iter().rev() {
                stack.push(*i);
            }
            visited.insert(last);
            write!(out, "{} ", last).unwrap();
        }
    }
    out.trim().to_string()
}

fn bfs(start: usize, graph: &[Vec<usize>]) -> String {
    let mut stack = VecDeque::new();
    stack.push_back(start);
    let mut visited = HashSet::new();
    let mut out = String::new();
    while !stack.is_empty() {
        let first = stack.pop_front().unwrap();
        if visited.contains(&first) {
            continue;
        } else {
            for i in &graph[first] {
                stack.push_back(*i);
            }
            visited.insert(first);
            write!(out, "{} ", first).unwrap();
        }
    }
    out.trim().to_string()
}
