use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());

    let mut out = String::new();
    for i in iter {
        if i == 0 {
            break;
        }

        let mut x: i32 = i;
        write!(out, "{x}").unwrap();
        while x > 9 {
            let mut y = x;
            let mut mul = y % 10;
            while y > 9 {
                y /= 10;
                mul *= y % 10;
            }

            x = mul;
            write!(out, " {x}").unwrap();
        }
        writeln!(out).unwrap();
    }

    print!("{out}");
}
