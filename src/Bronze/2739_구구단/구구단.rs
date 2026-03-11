use std::io;

fn main() {
    let mut buf  = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: usize = buf.trim().parse().unwrap();
    for i in 1..=9 {
        println!("{} * {} = {}", num, i, num * i);
    }
}