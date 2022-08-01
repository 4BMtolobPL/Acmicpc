use std::fmt::Write;

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut buf).unwrap();
    let mut v: Vec<i32> = buf.split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();
    v.sort();
    let mut out = String::new();
    for i in v {
        writeln!(&mut out, "{}", i).unwrap();
    }
    print!("{}", out);
}