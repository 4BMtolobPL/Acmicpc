use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let n = iter.next().unwrap();

    let mut t = vec![false; n];

    for i in iter {
        if t[i] {
            print!("{} ", i);
            break;
        } else {
            t[i] = true;
        }
    }
}
