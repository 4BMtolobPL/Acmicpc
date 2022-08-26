use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    iter.next();
    let m = iter.next().unwrap();
    let v: Vec<_> = iter.collect();
    let mut lhs: u32 = 0;
    let mut rhs = *v.iter().max().unwrap();
    let mut mid;

    while lhs <= rhs {
        mid = (lhs + rhs) / 2;
        let mut sum: u64 = 0;
        for &i in &v {
            if i > mid {
                sum += (i - mid) as u64;
            }
        }
        match sum.cmp(&(m as u64)) {
            std::cmp::Ordering::Less => rhs = mid - 1,
            std::cmp::Ordering::Equal => break,
            std::cmp::Ordering::Greater => lhs = mid + 1,
        }
    }
    mid = (lhs + rhs) / 2;

    println!("{}", mid);
}
