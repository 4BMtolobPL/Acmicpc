use std::collections::BTreeSet;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    let mut v: Vec<bool> = vec![true; 104];
    v[0] = false;
    v[1] = false;

    let mut primes = BTreeSet::new();
    let mut prev = 1;

    for i in 2..=103 {
        if v[i] {
            for j in (i * i..=103).step_by(i) {
                v[j] = false;
            }

            primes.insert(i * prev);
            prev = i;
        }
    }
    primes.pop_first().unwrap();

    println!("{}", primes.range((n + 1)..).next().unwrap());
}
