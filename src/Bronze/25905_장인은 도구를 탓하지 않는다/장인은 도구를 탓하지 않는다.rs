use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut v: Vec<f64> = buf
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    v.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let mut x = 1000000000.0;
    for i in 1..10 {
        x /= i as f64;
    }

    for i in v.iter().skip(1) {
        x *= i;
    }

    println!("{x}");
}
