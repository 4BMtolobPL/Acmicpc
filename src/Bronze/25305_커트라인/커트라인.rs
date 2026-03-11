use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let mut input = buf.split_whitespace().map(|x| x.parse().unwrap());
    let _: i32 = input.next().unwrap();
    let k = input.next().unwrap();

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let mut v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    v.sort();

    println!("{}", v[v.len() - k as usize]);
}