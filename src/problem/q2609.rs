use std::io::stdin;

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let v: Vec<u32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let gcf = gcd(v[0], v[1]);
    let lcm = v[0] * v[1] / gcf;
    println!("{}\n{}", gcf, lcm);
}

fn gcd(a: u32, b: u32) -> u32 {
    if b != 0 { gcd(b, a % b) } else { a }
}