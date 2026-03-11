use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let x: i32 = iter.next().unwrap();
    let y = iter.next().unwrap();
    let c = iter.next().unwrap();

    let diagonal_pow = (x * x) + (y * y);
    let distance = (diagonal_pow as f64).sqrt();

    if distance.fract() == 0.0 {
        // distance가 정수인경우
        let distance_i32 = distance.trunc() as i32;
        if distance_i32 % c == 0 {
            println!("{}", distance_i32 / c);
        } else if distance_i32 < c {
            println!("2");
        } else {
            println!("{}", distance_i32 / c + 1);
        }
    } else {
        // distance가 소수인경우 |distance| > 0

        if distance < c as f64 {
            println!("2");
        } else {
            println!("{}", (distance.ceil() / c as f64).ceil() as i32);
        }
    }
}
