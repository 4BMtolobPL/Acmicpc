use std::fmt::Write;
use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    println!("{}", biggest_number(buf.trim()));
    println!("{}", smallest_number(buf.trim()));
}

fn biggest_number(s: &str) -> String {
    let mut out = String::new();
    let mut b = 0;

    for c in s.chars() {
        if c == 'M' {
            b += 1
        } else if c == 'K' {
            write!(&mut out, "{}{}", 5, "0".repeat(b)).unwrap();
            b = 0;
        }
    }

    write!(&mut out, "{}", "1".repeat(b)).unwrap();

    out
}

fn smallest_number(s: &str) -> String {
    let mut out = String::new();
    let mut b = 0;

    for c in s.chars() {
        if c == 'M' {
            b += 1;
        } else if c == 'K' {
            if b > 0 {
                write!(&mut out, "1{}5", "0".repeat(b - 1)).unwrap();
                b = 0;
            } else {
                write!(&mut out, "5").unwrap();
            }
        }
    }

    if b > 0 {
        write!(&mut out, "1{}", "0".repeat(b - 1)).unwrap();
    }

    out
}
