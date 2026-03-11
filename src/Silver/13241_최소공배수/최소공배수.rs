use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let (a, b): (i64, i64) = {
        let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    println!("{}", (a * b).abs() / euclidean(a.max(b), a.min(b)));
}

fn euclidean(lhs: i64, rhs: i64) -> i64 {
    let r = lhs % rhs;

    if r == 0 { rhs } else { euclidean(rhs, r) }
}
