use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let a: i32 = iter.next().unwrap();
    let mut b = iter.next().unwrap();

    let mut count = 1;
    while a != b {
        if a > b {
            count = -1;
            break;
        } else if b % 10 == 1 {
            b /= 10;
            count += 1;
        } else if b % 2 == 0 {
            b /= 2;
            count += 1;
        } else {
            count = -1;
            break;
        }
    }

    println!("{count}");
}
