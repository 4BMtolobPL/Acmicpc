use std::io;

const MAX_A: usize = 10000;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
    let n = iter.next().unwrap();
    let m = iter.next().unwrap();

    let mut v = vec![0; MAX_A + 1];
    for i in iter {
        v[i] += 1;
    }

    if m % 2 == 0 {
        if v.iter().all(|x| x % 2 == 0) {
            println!("YES");
        } else {
            println!("NO");
        }
    } else if v.iter().filter(|x| *x % 2 != 0).count() <= n {
        println!("YES");
    } else {
        println!("NO");
    }
}
