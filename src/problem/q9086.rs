use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    for i in buf.split_whitespace().skip(1) {
        println!("{}{}", i.chars().next().unwrap(), i.chars().last().unwrap());
    }
}
