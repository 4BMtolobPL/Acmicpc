use std::io;

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let m: usize = iter.next().unwrap();
    let n: usize = iter.next().unwrap();

    let mut primearray = vec![true; n + 1];
    primearray[0] = false;
    primearray[1] = false;

    for i in (2..).take_while(|x| n >= x * x) {
        if primearray[i] {
            for j in (i * i..=n).step_by(i) {
                primearray[j] = false;
            }
        }
    }

    let mut out = String::new();
    for i in m..=n {
        if primearray[i] {
            out.push_str(&format!("{}\n", i));
        }
    }

    println!("{}", out.trim());
}