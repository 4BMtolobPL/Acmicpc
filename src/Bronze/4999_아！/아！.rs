use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace();
    if iter.next().unwrap().len() >= iter.next().unwrap().len() {
        println!("go");
    } else {
        println!("no");
    }
}