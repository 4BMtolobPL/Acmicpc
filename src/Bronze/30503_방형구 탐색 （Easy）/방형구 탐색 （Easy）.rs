use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();
    let _n = lines.next().unwrap();

    let mut v: Vec<i32> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let _q = lines.next().unwrap();

    let mut out = String::new();

    for line in lines {
        let mut iter = line.split_ascii_whitespace().map(|x| x.parse().unwrap());
        let query: i32 = iter.next().unwrap();

        if query == 1 {
            let l = iter.next().unwrap() as usize - 1;
            let r = iter.next().unwrap() as usize - 1;
            let k = iter.next().unwrap();

            let mut count = 0;
            if let Some(slice) = v.get(l..=r) {
                for i in slice {
                    if *i == k {
                        count += 1;
                    }
                }
            }

            writeln!(out, "{count}").unwrap();
        } else {
            let l = iter.next().unwrap() as usize - 1;
            let r = iter.next().unwrap() as usize - 1;

            if let Some(slice) = v.get_mut(l..=r) {
                for i in slice {
                    *i = -1;
                }
            }
        }
    }

    print!("{out}");
}
