use std::io::stdin;

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let a: i64 = iter.next().unwrap();
    let b = iter.next().unwrap();

    let mut x = 0;
    if b - a < 9 {
        for i in a..=b {
            x ^= i;
        }
    } else {
        for i in a..(a + 4 - (a % 4)) {
            x ^= i;
        }
        for j in (b - b % 4)..=b {
            x ^= j;
        }
    }

    println!("{}", x);
}
