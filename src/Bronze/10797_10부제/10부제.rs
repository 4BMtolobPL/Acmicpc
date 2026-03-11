use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());

    let day: i32 = iter.next().unwrap();

    println!("{}", iter.filter(|x| *x % 10 == day).count());
}
