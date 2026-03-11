use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    // n: 1보다 크고 500보다 작은 홀수
    let n: usize = buf.trim_end().parse().unwrap();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; n + 1]; n + 1];
    for (i, row) in visited.iter_mut().enumerate() {
        row[i] = true;
    }
    visited[1][n] = true;
    visited[n][1] = true;

    let mut next_index = vec![1; n + 1];

    let mut out = String::new();
    write!(&mut out, "a1").unwrap();

    let mut now = 1;
    for _ in 0..(n * (n - 1) / 2 - 1) {
        for (next, value) in visited[now].iter_mut().enumerate().skip(next_index[now]) {
            if !*value {
                *value = true;
                visited[next][now] = true;
                next_index[now] = next + 1;

                write!(&mut out, " a{next}").unwrap();
                now = next;

                break;
            }
        }
    }

    println!("{out} a1");
}
