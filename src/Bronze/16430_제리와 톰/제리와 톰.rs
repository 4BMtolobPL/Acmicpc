use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let a: i32 = iter.next().unwrap();
    let b = iter.next().unwrap();

    println!("{} {}", b - a, b);
}