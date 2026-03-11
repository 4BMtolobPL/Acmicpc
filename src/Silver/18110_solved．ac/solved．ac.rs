use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse::<_>().unwrap());
    let n = iter.next().unwrap();
    let end = (n as f64 * 0.15).round() as usize;

    let mut opinions: Vec<_> = iter.collect();
    opinions.sort_unstable();

    println!(
        "{}",
        (opinions.iter().take(n - end).skip(end).sum::<usize>() as f64 / (n - (end * 2)) as f64)
            .round() as usize
    );
}
