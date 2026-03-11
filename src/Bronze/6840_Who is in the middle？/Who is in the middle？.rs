use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());

    let a: i32 = iter.next().unwrap();
    let b = iter.next().unwrap();
    let c = iter.next().unwrap();

    if a > b {
        if a > c {
            println!("{}", b.max(c));
        } else {
            println!("{a}");
        }
    } else if b > c {
        println!("{}", a.max(c));
    } else {
        println!("{b}");
    }
}
