use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let mut out = String::new();

    loop {
        let (n, m) = (iter.next().unwrap(), iter.next().unwrap());

        if n == 0 {
            break;
        }

        let mut v = Vec::new();
        for _ in 0..n {
            let (start, duration) = (iter.by_ref().nth(2).unwrap(), iter.next().unwrap());

            v.push((start, start + duration));
        }

        for _ in 0..m {
            let (start, duration) = (iter.next().unwrap(), iter.next().unwrap());
            let end = start + duration;
            let count = v.iter().filter(|(l, r)| !(start >= *r || end <= *l)).count();

            writeln!(&mut out, "{count}").unwrap();
        }
    }

    print!("{out}");
}
