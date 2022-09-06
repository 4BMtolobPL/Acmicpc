use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    lines.next();
    let s: Vec<char> = lines.next().unwrap().trim_matches('O').chars().collect();

    let mut start = 0;

    let mut v = Vec::new();

    while start < s.len() {
        if s[start] == 'I' {
            let mut count = 0;
            while start < s.len() - 2 {
                if s[start + 1] == 'O' && s[start + 2] == 'I' {
                    start += 2;
                    count += 1;
                } else {
                    break;
                }
            }
            if count > 0 {
                v.push(count);
            }
            start += 1;
        } else {
            start += 1;
        }
    }

    let mut sum = 0;
    for i in v {
        if i >= n {
            sum += i + 1 - n;
        }
    }

    println!("{}", sum);
}
