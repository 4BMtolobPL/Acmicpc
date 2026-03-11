use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let mut sum: i32 = buf.trim().parse().unwrap();

    buf.clear();
    stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();

    for _ in 0..n {
        buf.clear();
        stdin().read_line(&mut buf).unwrap();
        let v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
        sum -= v[0] * v[1];
    }

    println!("{}", if sum == 0 { "Yes" } else { "No" });
}