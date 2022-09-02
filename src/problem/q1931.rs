use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut v: Vec<(usize, usize)> = Vec::new();
    for line in buf.lines().skip(1) {
        let (start, end) = line.trim().split_once(' ').unwrap();
        v.push((start.parse().unwrap(), end.parse().unwrap()));
    }

    v.sort_unstable_by(|lhs, rhs| match lhs.1.cmp(&rhs.1) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => lhs.0.cmp(&rhs.0),
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    });

    let mut count = 0;
    let mut last = 0;
    for (start, end) in v {
        if last <= start {
            last = end;
            count += 1;
        }
    }

    println!("{}", count);
}
