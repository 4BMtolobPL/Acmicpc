use std::io;

fn main() {
    let mut buf  = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    println!("{}", buf.as_bytes()[0]);
}