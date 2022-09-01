use std::io::Write;
use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter},
};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let n: usize = iter.next().unwrap();
    let k = iter.next().unwrap();

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);

    let mut queue = VecDeque::new();
    let mut visited: Vec<Option<usize>> = vec![None; 100001];
    queue.push_back(n);
    visited[n] = Some(0);

    while !queue.is_empty() {
        let first = queue.pop_front().unwrap();
        let time = visited[first].unwrap();

        if first * 2 <= 100000 {
            branch(first * 2, time, &mut visited, &mut queue);
        }

        if first < 100000 {
            branch(first + 1, time, &mut visited, &mut queue);
        }

        if first > 0 {
            branch(first - 1, time, &mut visited, &mut queue);
        }
    }
    writeln!(out, "{}", visited[k].unwrap()).unwrap();
}

fn branch(next: usize, time: usize, visited: &mut [Option<usize>], queue: &mut VecDeque<usize>) {
    match visited[next] {
        Some(x) => {
            if time + 1 < x {
                visited[next] = Some(time + 1);
                queue.push_back(next);
            }
        }
        None => {
            visited[next] = Some(time + 1);
            queue.push_back(next);
        }
    }
}
