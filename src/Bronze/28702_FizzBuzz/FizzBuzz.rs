use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let iter = buf.split_whitespace().map(|x| x.parse::<i32>());
    for (i, d) in iter.enumerate() {
        if let Ok(n) = d {
            let x = n + 3 - i as i32;
            if x % 3 == 0 {
                print!("Fizz");
            }

            if x % 5 == 0 {
                print!("Buzz");
            }

            if x % 3 != 0 && x % 5 != 0 {
                print!("{}", x);
            }

            break;
        }
    }
}
