use std::io;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let s: Vec<char> = iter.next().unwrap().chars().collect();

    let mut prev_right = 0;
    let mut total = 0;

    for (i, c) in s.iter().enumerate() {
        if *c == 'R' {
            let left = i.saturating_sub(k);
            let right = n.min(i + k + 1);

            total += right - left.max(prev_right);

            prev_right = right;
        }
    }
    
    println!("{}", if total > m { "No" } else { "Yes" });
}
