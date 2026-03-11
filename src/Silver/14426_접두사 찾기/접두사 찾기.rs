use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();

    let (n, m): (usize, usize) = {
        (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        )
    };

    let mut t: Vec<_> = iter.by_ref().take(n).collect();
    let v: Vec<_> = iter.by_ref().take(m).collect();
    t.sort_unstable();

    println!(
        "{}",
        v.iter().fold(0, |acc, &x| {
            if contains_starts_with(x, &t) {
                acc + 1
            } else {
                acc
            }
        })
    );
}

fn contains_starts_with(s: &str, from: &[&str]) -> bool {
    let mut lhs = 0;
    let mut rhs = from.len();
    while lhs < rhs {
        let center = (lhs + rhs) / 2;

        if from[center].starts_with(s) {
            return true;
        }

        match from[center].cmp(s) {
            std::cmp::Ordering::Less => {
                lhs = center + 1;
            }
            std::cmp::Ordering::Equal => {
                return true;
            }
            std::cmp::Ordering::Greater => rhs = center,
        }
    }

    false
}
