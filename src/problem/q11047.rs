use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    iter.next();
    let mut k: i32 = iter.next().unwrap();

    let mut count = 0;
    for coin in iter.rev() {
        count += k / coin;
        k %= coin;
    }

    println!("{}", count);
}
