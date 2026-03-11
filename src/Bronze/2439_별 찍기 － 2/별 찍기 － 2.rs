use std::io;

fn main() {
    let mut buf  = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: usize = buf.trim().parse().unwrap();
    (1..=num).for_each(|x| println!("{}{}", " ".repeat(num - x), "*".repeat(x)));
}