use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let (n, mut k): (_, i32) = {
        let mut line = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap());
        (line.next().unwrap(), line.next().unwrap())
    };

    for i in lines.skip(1).map(|x| x.parse().unwrap()) {
        if i > 0 {
            if k > i {
                continue;
            } else {
                k = i - k + 1
            }
        } else if k <= n + i {
            continue;
        } else {
            k = n + n - k + i + 1;
        }
    }
    println!("{}", k)
}
