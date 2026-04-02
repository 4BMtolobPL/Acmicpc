use std::fmt::Write;
use std::io;
use std::io::Read;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let t: usize = iter.next().unwrap();

    let mut out = String::new();
    for _ in 0..t {
        let (m, n) = (iter.next().unwrap(), iter.next().unwrap());

        let mut v = vec![0; n];
        let mut count = 0;

        for _ in 0..m {
            for index in 0..n {
                let cell = iter.next().unwrap();

                if cell == 1 {
                    v[index] += 1;
                } else {
                    count += v[index];
                }
            }
        }

        writeln!(out, "{count}").unwrap();
    }

    print!("{out}");
}
