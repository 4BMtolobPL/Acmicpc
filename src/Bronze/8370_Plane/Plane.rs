use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n1: i32 = iter.next().unwrap();
    let k1 = iter.next().unwrap();
    let n2 = iter.next().unwrap();
    let k2 = iter.next().unwrap();

    println!("{}", n1 * k1 + n2 * k2);
}
