use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut v: Vec<(i32, &str)> = buf
        .lines()
        .skip(1)
        .map(|x| x.split_at(x.find(char::is_whitespace).unwrap()))
        .map(|(a, b)| (a.parse().unwrap(), b))
        .collect();
    v.sort_by(|a, b| a.0.cmp(&b.0));

    let mut out = Vec::new();
    for (a, b) in v {
        out.push(format!("{}{}", a, b));
    }
    println!("{}", out.join("\n"));
}
