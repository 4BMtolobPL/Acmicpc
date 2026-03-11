use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());

    let n: usize = iter.next().unwrap();
    let mut v: Vec<usize> = iter.collect();

    v.sort_unstable();
    let mut sum = v[(n / 2)..n].iter().sum::<usize>() * 2;

    if n % 2 == 1 {
        sum -= v[n / 2];
    }

    println!("{sum}");
}
