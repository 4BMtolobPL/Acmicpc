use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut s = buf.split_whitespace();
    let (_, n): (_, i32) = (s.next(), s.next().unwrap().parse().unwrap());

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    v.retain(|x| *x < n);

    for i in v {
        print!("{} ", i);
    }
}