use std::io;

fn main() {
    let mut buf  = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: f64 = buf.trim().parse().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let max: f64 = *v.iter().max().unwrap() as f64;
    let sum: i32 = v.iter().sum();
    println!("{}", sum as f64 * 100.0 / max / n)
}