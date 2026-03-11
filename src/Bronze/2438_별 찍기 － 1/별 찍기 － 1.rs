use std::io::{self, Write};

fn main() {
    let mut buf  = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let num: i32 = buf.trim().parse().unwrap();

    let mut line = String::new();

    for i in 1..(num + 1) {
        for _ in 0..i {
            line.push('*');
        }
        line.push('\n');
    }

    let mut stdout = io::stdout();
    write!(stdout, "{}", line).unwrap();

}