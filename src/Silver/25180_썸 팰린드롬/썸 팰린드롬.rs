use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    let mut sum = n / 18 * 2;
    let b = n % 18;
    if b == 0 {
    } else if b < 10 {
        sum += 1;
    } else if b % 2 == 0 {
        sum += 2;
    } else {
        sum += 3;
    }

    println!("{sum}");
}
