use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n = iter.next().unwrap();
    let m = iter.next().unwrap();
    let mut v = vec![0; n + 1];

    for i in 0..n {
        v[i + 1] = iter.next().unwrap();
    }
    for _ in 0..m {
        v[0] = iter.next().unwrap();
        for i in 1..=n {
            v[i] = if v[i - 1] == v[i] { 0 } else { 1 };
        }
    }

    println!("{}", v[n]);
}
