use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let (_n, mut a, mut b): (usize, _, _) = {
        let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
        (
            iter.next().unwrap(),
            iter.next().unwrap() - 1,
            iter.next().unwrap() - 1,
        )
    };

    for count in 1.. {
        a /= 2;
        b /= 2;

        if a == b {
            println!("{count}");
            break;
        }
    }
}
