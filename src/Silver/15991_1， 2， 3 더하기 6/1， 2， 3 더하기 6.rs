use std::fmt::Write;
use std::io;

fn main() {
    const DIVIDER: usize = 1000000009;
    let pre = [1, 1, 2, 2, 3, 3, 6, 6, 11, 11, 20];
    let mut v = vec![0; 100001];

    for (i, value) in pre.iter().enumerate() {
        v[i] = *value
    }

    for i in pre.len()..v.len() {
        v[i] = ((v[i - 6] + v[i - 4]) % DIVIDER + v[i - 2]) % DIVIDER;
    }

    let buf = io::read_to_string(io::stdin()).unwrap();
    let iter = buf
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap());

    let mut out = String::new();
    for n in iter {
        writeln!(&mut out, "{}", v[n]).unwrap();
    }

    print!("{}", out);
}
