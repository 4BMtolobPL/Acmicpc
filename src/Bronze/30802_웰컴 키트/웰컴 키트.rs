use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse::<i32>().unwrap());

    let n = iter.next().unwrap();
    let claimants: Vec<_> = iter.by_ref().take(6).collect();
    let (t, p) = (iter.next().unwrap(), iter.next().unwrap());

    let mut shirts = 0;
    for s in claimants.iter() {
        shirts += s / t;
        if (s % t) != 0 {
            shirts += 1;
        }
    }

    println!("{}\n{} {}", shirts, n / p, n % p);
}
