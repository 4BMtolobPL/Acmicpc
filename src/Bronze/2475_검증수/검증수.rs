use std::io;

fn main() {
    let mut buf  = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut x = 0;

    for i in 0..5 {
        x += v[i] * v[i];
    }

    println!("{}", x % 10);
}