use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    println!("{}", (n + 1) * n / 2);
}
