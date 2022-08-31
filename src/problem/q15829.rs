use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    lines.next();
    let input = lines.next().unwrap();

    println!("{}", hash(input));
}

fn hash(src: &str) -> i64 {
    let mut sum: i64 = 0;
    for (index, c) in src.chars().enumerate() {
        let mut a = c as i64 - 'a' as i64 + 1;
        for _ in 0..index {
            a = (a * 31) % 1234567891
        }
        sum += a;
    }
    sum % 1234567891
}
