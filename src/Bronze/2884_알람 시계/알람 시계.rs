use std::io;

fn main() {
    let mut buf  = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let time = (1440 + (v[0] * 60) + v[1] - 45) % 1440;
    println!("{} {}", time / 60, time % 60);
}