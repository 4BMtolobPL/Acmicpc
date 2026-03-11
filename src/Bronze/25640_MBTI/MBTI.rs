use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();
    let mbti = lines.next().unwrap();

    println!("{}", lines.skip(1).filter(|l| *l == mbti).count());
}
