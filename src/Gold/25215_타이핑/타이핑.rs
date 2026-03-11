use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut last = (0, 1);

    for c in buf.trim().chars() {
        if c.is_uppercase() {
            // 대문자 입력
            let l = last.0.min(last.1) + 2;
            let u = (last.0 + 2).min(last.1 + 1);

            last = (l, u);
        } else {
            // 소문자 입력
            let l = (last.0 + 1).min(last.1 + 2);
            let u = last.0.min(last.1) + 2;

            last = (l, u);
        }
    }

    println!("{}", last.0.min(last.1));
}
