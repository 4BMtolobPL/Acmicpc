use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let (n, m, k): (usize, _, _) = {
        let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    let min = n.saturating_sub(m * k);
    let max = (n - 1).saturating_sub(m * (k - 1));
    println!("{min} {max}");
}
