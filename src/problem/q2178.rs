use std::{
    collections::VecDeque,
    io::{stdin, Read},
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
    let n = iter.next().unwrap();
    let m = iter.next().unwrap();

    let mut map = Vec::with_capacity(n);
    for line in lines {
        let v: Vec<bool> = line.trim().chars().map(|x| x == '1').collect();
        map.push(v);
    }

    println!("{}", bfs(n, m, map));
}

fn bfs(n: usize, m: usize, map: Vec<Vec<bool>>) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    let mut visited = vec![vec![false; m]; n];
    let mut count = 0;
    while !queue.is_empty() {
        count += 1;

        for _ in 0..queue.len() {
            let (x, y) = queue.pop_front().unwrap();
            if x == (n - 1) && y == (m - 1) {
                return count;
            }

            let nexts = get_nexts(x, y, n, m);

            for next in nexts {
                if !visited[next.0][next.1] && map[next.0][next.1] {
                    visited[next.0][next.1] = true;
                    queue.push_back((next.0, next.1));
                }
            }
        }
    }
    -1
}

fn get_nexts(x: usize, y: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
    let mut next = Vec::new();
    if x > 0 {
        next.push((x - 1, y));
    }
    if y > 0 {
        next.push((x, y - 1));
    }
    if x < n - 1 {
        next.push((x + 1, y));
    }
    if y < m - 1 {
        next.push((x, y + 1));
    }
    next
}
