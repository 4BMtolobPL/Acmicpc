use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    iter.next();
    let m = iter.next().unwrap();
    let v: Vec<i32> = iter.collect();

    let mut nearest = 0;

    for i in 0..(v.len() -2) {
        for j in (i+1)..(v.len() - 1) {
            for k in (j+1)..v.len() {
                let sum = v[i] + v[j] + v[k];
                if m - sum >= 0 && (m - nearest) > (m - sum) {
                    nearest = sum;
                }
            }
        }
    }

    println!("{}", nearest);
}