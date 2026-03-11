use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let (r, c, k): (usize, usize, usize) = {
        let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    if (r % 2 == 0 || c % 2 == 0) || k == 1 {
        println!("1");
    } else {
        println!("0");
    }
}
