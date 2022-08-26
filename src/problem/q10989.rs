use std::io::{stdin, stdout, BufWriter, Write};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse().unwrap();

    let mut v = [0; 10001];
    for _ in 0..n {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();
        let index: usize = buf.trim().parse().unwrap();
        v[index] += 1;
    }

    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());
    for (index, &value) in v.iter().enumerate() {
        for _ in 0..value {
            writeln!(out, "{}", index).unwrap();
        }
    }
}
