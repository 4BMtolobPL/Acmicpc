use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    for line in buf.lines().skip(1) {
        let mut iter = line.split_whitespace().map(|x| x.parse().unwrap());
        let (m, n, x, y) = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );

        let (gcd, mut k0, _) = extend_euclidean(m, n);

        if (y - x) % gcd != 0 {
            println!("-1");
        } else {
            k0 *= (y - x) / gcd;
            let mut k = (m * k0 + x) % (m * n / gcd);
            while k <= 0 {
                k += m * n / gcd;
            }
            println!("{}", k);
        }
    }
}

fn extend_euclidean(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x, y) = extend_euclidean(b, a % b);
        (gcd, y, x - (a / b) * y)
    }
}
