use std::io::stdin;

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let a: i32 = iter.next().unwrap();
    let b = iter.next().unwrap();
    let v = iter.next().unwrap();

    let daily_up = a - b;
    let x = v - a;
    let y = x / daily_up + if x % daily_up != 0 { 1 } else { 0 };
    println!("{}", y + 1);
}
