use std::io;

fn main() {
    let mut buf  = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("{} {}", v.iter().min().unwrap(), v.iter().max().unwrap());
}