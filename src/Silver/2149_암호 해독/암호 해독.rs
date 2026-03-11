use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let (key, enc) = (lines.next().unwrap(), lines.next().unwrap());

    let mut key_index: Vec<(_, _)> = key.chars().enumerate().collect();
    key_index.sort_by(|a, b| a.1.cmp(&b.1));
    let mut order: Vec<(_, _)> = key_index.iter().enumerate().map(|x| (x.0, x.1.0)).collect();
    order.sort_by(|a, b| a.1.cmp(&b.1));
    let chars: Vec<_> = enc.chars().collect();

    let mut out = String::new();
    for i in 0..(enc.len() / key.len()) {
        for (j, _) in order.iter() {
            out.push(chars[i + j * (enc.len() / key.len())]);
        }
    }

    println!("{}", out);
}
