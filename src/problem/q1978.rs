use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    const TOP: usize = 1000;
    let sqrt = (TOP as f64).sqrt() as usize;
    let mut primes = [true; TOP + 1];
    primes[0] = false;
    primes[1] = false;
    for i in 2..=(sqrt + 1) {
        if primes[i] {
            for j in ((i * i)..=TOP).step_by(i) {
                primes[j] = false;
            }
        }
    }

    println!(
        "{}",
        buf.split_whitespace()
            .skip(1)
            .map(|x| x.parse::<usize>().unwrap())
            .filter(|&x| primes[x])
            .count()
    );
}
