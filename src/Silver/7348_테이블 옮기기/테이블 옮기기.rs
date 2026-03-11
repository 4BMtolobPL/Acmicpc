use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();

    let testcase = lines.next().unwrap().parse().unwrap();

    let mut out = String::new();
    for _ in 0..testcase {
        let n = lines.next().unwrap().parse().unwrap();

        let mut v: Vec<i32> = vec![0; 201];
        for _ in 0..n {
            let mut iter = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap());
            let from: usize = iter.next().unwrap();
            let to = iter.next().unwrap();

            v[(from.min(to) - 1) / 2] += 1;
            v[(from.max(to) - 1) / 2 + 1] -= 1;
        }

        let mut max = 0;
        let mut sum = 0;
        for i in v.iter() {
            sum += i;
            max = max.max(sum);
        }

        writeln!(out, "{max}0").unwrap();
    }

    print!("{out}");
}
