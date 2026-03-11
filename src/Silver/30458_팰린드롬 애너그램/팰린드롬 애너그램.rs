use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let s = buf.split_ascii_whitespace().nth(1).unwrap();

    let middle = if s.len() % 2 == 0 {
        None
    } else {
        Some(s.len() / 2)
    };

    let mut v = [false; 26];

    for (i, c) in s.as_bytes().iter().enumerate() {
        if let Some(m) = middle {
            if m == i {
                continue;
            }
        }

        v[(c - b'a') as usize] = !v[(c - b'a') as usize];
    }

    print!("{}", if v.iter().all(|&b| !b) { "Yes" } else { "No" });
}
