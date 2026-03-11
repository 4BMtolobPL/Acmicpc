use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let test_case: i32 = buf.trim().parse().unwrap();

    for _ in 0..test_case {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();
        split_call_mul(buf.trim());
    }
}

fn str_mul(r: i32, s: &str) -> String {
    let mut result = String::new();

    for c in s.chars() {
        for _ in 0..r {
            result.push(c);
        }
    }

    return result;
}

fn split_call_mul(s: &str) {
    let mut h = s.split_whitespace();
    let r: i32 = h.next().unwrap().parse().unwrap();
    let s = h.next().unwrap();

    println!("{}", str_mul(r, s));
}