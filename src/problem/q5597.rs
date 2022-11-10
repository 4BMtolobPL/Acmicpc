use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut v = [false; 31];
    for s in buf.split_whitespace() {
        v[s.parse::<usize>().unwrap()] = true;
    }

    for (index, value) in v.iter().enumerate().skip(1) {
        if !value {
            println!("{}", index);
        }
    }
}
