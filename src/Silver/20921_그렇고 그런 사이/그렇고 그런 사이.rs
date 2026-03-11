use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let mut n: usize = iter.next().unwrap(); // 2 <= n <= 4242
    let mut k = iter.next().unwrap(); // 0 <= k <= n * (n - 1) / 2

    let mut v = Vec::new();
    let mut used = vec![false; n];

    loop {
        if k == 0 {
            let mut left: Vec<usize> = used
                .iter()
                .enumerate()
                .filter_map(|(index, value)| if *value { None } else { Some(index + 1) })
                .collect();
            v.append(&mut left);
            break;
        } else if k >= n {
            v.push(n);
            used[n - 1] = true;
        } else {
            v.push(k + 1);
            used[k] = true;
        }
        n -= 1;
        k = k.saturating_sub(n);
    }

    let mut iter = v.iter();
    let mut out = String::new();
    write!(out, "{}", iter.next().unwrap()).unwrap();
    for i in iter {
        write!(out, " {i}").unwrap();
    }
    writeln!(out).unwrap();
    print!("{out}");
}
