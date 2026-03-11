use std::fmt::Write;
use std::io;

const DIV: i32 = 1_000_000_009;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());

    let mut ans = vec![1, 1, 2, 4, 7, 13, 24, 44, 81, 149, 274];

    let mut out = String::new();
    for n in iter.skip(1) {
        calc(&mut ans, n);

        writeln!(out, "{}", ans[n]).unwrap();
    }

    print!("{out}");
}

fn calc(v: &mut Vec<i32>, n: usize) {
    let len = v.len();

    if len > n {
        return;
    }

    for i in len..=n {
        v.push((((v[i - 3] + v[i - 2]) % DIV) + v[i - 1]) % DIV);
    }
}
