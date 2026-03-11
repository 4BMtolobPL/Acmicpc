use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let n = buf.trim().parse().unwrap();

    let mut s = String::new();
    for i in 1..=n {
        s.push_str(&i.to_string());
    }

    println!("{}", s.find(n.to_string().as_str()).unwrap() + 1);
}
