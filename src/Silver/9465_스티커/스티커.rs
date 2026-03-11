use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let testcase: usize = lines.next().unwrap().parse().unwrap();
    let mut out = String::new();
    for _ in 0..testcase {
        let n: usize = lines.next().unwrap().parse().unwrap();
        let v: Vec<Vec<i32>> = lines
            .by_ref()
            .take(2)
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .collect();

        let mut max = vec![vec![0; n]; 2];
        max[0][0] = v[0][0];
        max[1][0] = v[1][0];
        if n > 1 {
            max[0][1] = max[1][0] + v[0][1];
            max[1][1] = max[0][0] + v[1][1];
        }
        for i in 2..n {
            max[0][i] = max[1][i - 2].max(max[1][i - 1]) + v[0][i];
            max[1][i] = max[0][i - 2].max(max[0][i - 1]) + v[1][i];
        }

        writeln!(out, "{}", max[0][n - 1].max(max[1][n - 1])).unwrap();
    }

    print!("{out}");
}
