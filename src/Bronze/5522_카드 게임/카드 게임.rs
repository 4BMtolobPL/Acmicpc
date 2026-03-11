use std::io::Write;
use std::io::{stdin, stdout, BufWriter, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let (len, mut s1, mut s2, mut s3, mut s4, mut s5) = {
        let len = buf
            .split_whitespace()
            .max_by(|x, y| x.len().cmp(&y.len()))
            .unwrap()
            .len();
        let mut iter = buf
            .split_whitespace()
            .map(|s| s.chars().map(|x| x.to_digit(10).unwrap()).rev());
        (
            len,
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    let mut carry = 0;
    let mut v = Vec::new();
    for _ in 0..len {
        let sum = s1.next().unwrap_or(0)
            + s2.next().unwrap_or(0)
            + s3.next().unwrap_or(0)
            + s4.next().unwrap_or(0)
            + s5.next().unwrap_or(0)
            + carry;

        carry = sum / 10;
        v.push(sum % 10);
    }
    if carry != 0 {
        v.push(carry);
    }

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);
    v.iter().rev().for_each(|x| write!(out, "{}", x).unwrap());
}