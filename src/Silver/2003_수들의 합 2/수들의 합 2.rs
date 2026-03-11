use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n = iter.next().unwrap() as usize;
    let m = iter.next().unwrap();
    let v: Vec<i32> = iter.collect();

    let mut count = 0;
    let mut sum = v[0];
    let mut lhs = 0;
    let mut rhs = 0;

    while rhs < n {
        if sum == m {
            count += 1;
        }

        if sum < m {
            rhs += 1;
            if let Some(x) = v.get(rhs) {
                sum += x;
            }
        } else {
            sum -= v[lhs];
            lhs += 1;
            if lhs > rhs {
                rhs += 1;
                if let Some(x) = v.get(rhs) {
                    sum += x;
                }
            }
        }
    }

    println!("{count}");
}
