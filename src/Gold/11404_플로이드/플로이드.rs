use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut shortest = vec![vec![i32::MAX; n + 1]; n + 1];

    for (i, val) in shortest.iter_mut().enumerate().skip(1) {
        if let Some(x) = val.get_mut(i) {
            *x = 0;
        }
    }

    for _ in 0..m {
        let from: usize = iter.next().unwrap().parse().unwrap();
        let to: usize = iter.next().unwrap().parse().unwrap();
        let cost: i32 = iter.next().unwrap().parse().unwrap();

        if shortest[from][to] > cost {
            shortest[from][to] = cost;
        }
    }

    for middle in 1..=n {
        for from in 1..=n {
            for to in 1..=n {
                shortest[from][to] = shortest[from][to]
                    .min(shortest[from][middle].saturating_add(shortest[middle][to]));
            }
        }
    }

    let mut out = String::new();
    for row in shortest.iter().skip(1) {
        let mut iter = row.iter().skip(1);

        if let Some(first) = iter.next() {
            if *first == i32::MAX {
                write!(out, "0",).unwrap();
            } else {
                write!(out, "{first}",).unwrap();
            }
        }

        for i in iter {
            if *i == i32::MAX {
                write!(out, " 0",).unwrap();
            } else {
                write!(out, " {i}",).unwrap();
            }
        }
        writeln!(out).unwrap();
    }

    print!("{out}");
}
