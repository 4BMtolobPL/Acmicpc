use std::{io::{Read, stdin}, collections::HashSet};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let v: Vec<usize> = buf.split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();
    let primes = get_primes(1000);

    println!("{}", v.iter().filter(|x| primes.contains(*x)).count());
}

fn get_primes(rhs: usize) -> HashSet<usize> {
    let mut nums = vec![true; rhs + 1];
    for i in 2..=32 {
        if nums[i] {
            for j in ((i * i)..=rhs).step_by(i) {
                nums[j] = false;
            }
        }
    }

    let mut primes = HashSet::new();
    for (index, value) in nums.iter().enumerate().skip(2) {
        if *value {
            primes.insert(index);
        }
    }
    primes
}