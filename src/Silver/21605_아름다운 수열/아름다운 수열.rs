use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let n: usize = buf.trim_end().parse().unwrap();

    let com = ["1 1 -1 -1 1 -1 -1 1", "-1 1", "1 -1 -1 1", "1 1 -1 -1 -1 1"];

    let m = n % 4;
    let d = n / 4;

    let mut out = Vec::new();
    if m != 0 {
        out.push(com[m]);
    }

    for _ in 0..d {
        out.push(com[0]);
    }

    println!("{}", out.join(" "));
}
