use std::cmp::max;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let a = lines.next().unwrap();
    let b = lines.next().unwrap();

    println!("{}", lcs_len(a, b));
}

fn lcs_len(lhs: &str, rhs: &str) -> usize {
    let l_chars: Vec<char> = lhs.chars().collect();
    let r_chars: Vec<char> = rhs.chars().collect();

    let mut start = 0;
    let mut end_l = l_chars.len();
    let mut end_r = r_chars.len();
    while start < l_chars.len() && start < r_chars.len() && l_chars[start] == r_chars[start] {
        start += 1;
    }
    while start < end_l && start < end_r && l_chars[end_l - 1] == r_chars[end_r - 1] {
        end_l -= 1;
        end_r -= 1;
    }

    let (short, long) = if l_chars.len() <= r_chars.len() {
        (&l_chars[start..end_l], &r_chars[start..end_r])
    } else {
        (&r_chars[start..end_r], &l_chars[start..end_l])
    };

    let mut prev: Vec<usize> = vec![0; short.len() + 1];

    for i in 1..=long.len() {
        let mut v = vec![0; short.len() + 1];
        for j in 1..=short.len() {
            if short[j - 1] == long[i - 1] {
                v[j] = prev[j - 1] + 1;
            } else {
                v[j] = max(prev[j], v[j - 1]);
            }
        }
        prev = v;
    }

    prev[short.len()] + start + (l_chars.len() - end_l)
}
