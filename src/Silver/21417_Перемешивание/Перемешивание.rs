use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace();

    let s = iter.next().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut suffled = s.to_string();
    for _ in 0..k {
        suffled = suffle(&suffled);
    }

    println!("{suffled}");
}

fn suffle(s: &str) -> String {
    let lhs = s.chars().rev().step_by(2);
    let rhs = s.chars().rev().skip(1).step_by(2);

    rhs.chain(lhs).collect()
}
