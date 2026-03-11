use std::io;

fn main() {
    let mut buf  = String::new();
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..9 {
        io::stdin().read_line(&mut buf).unwrap();
        v.push(buf.trim().parse().unwrap());
        buf.clear();
    }
    let max = v.iter().max().unwrap();
    println!("{}", max);
    println!("{}", v.iter().position(|x| x == max).unwrap() + 1);
}