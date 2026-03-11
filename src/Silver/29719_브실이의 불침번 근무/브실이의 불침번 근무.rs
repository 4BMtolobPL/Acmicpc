use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let (n, m): (u64, _) = {
        let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    const MOD: u64 = 1000000007;
    let mut max = 1;
    let mut min = 1;

    for _ in 0..n {
        max *= m;
        max %= MOD;

        min *= m - 1;
        min %= MOD;
    }

    println!("{}", (max + MOD - min) % MOD);
}
