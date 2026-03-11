use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut index = 0;
    let sum = buf
        .trim_end()
        .chars()
        .enumerate()
        .map(|(i, x)| {
            if x == '*' {
                index = i;
                0
            } else {
                let r = x.to_digit(10).unwrap();
                if i % 2 == 1 { r * 3 } else { r }
            }
        })
        .sum::<u32>();

    let result = if index % 2 == 0 {
        (10 - (sum % 10)) % 10
    } else {
        let mut x = (10 - (sum % 10)) % 10;
        while x % 3 != 0 {
            x += 10;
        }

        x / 3
    };

    println!("{result}");
}
