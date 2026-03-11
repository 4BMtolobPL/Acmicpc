use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();

    let iter = buf.trim_end().chars().map(|x| x.to_digit(10).unwrap());

    let mut v = Vec::new();
    let mut sum = 0;
    for i in iter {
        v.push(sum);
        sum += i;
    }
    v.push(sum);

    let mut max = 0;
    for lhs in 0..v.len() {
        for rhs in ((lhs + 2)..v.len()).rev() {
            let len = rhs - lhs;
            if len % 2 != 0 {
                continue;
            }

            if len <= max {
                continue;
            }

            if v[lhs + (len / 2)] - v[lhs] == v[rhs] - v[rhs - (len / 2)] {
                max = len;
            }
        }
    }

    println!("{max}");
}
