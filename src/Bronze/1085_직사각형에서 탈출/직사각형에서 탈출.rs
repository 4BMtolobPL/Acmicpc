fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let s: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut v = Vec::new();
    v.push(s[2] - s[0]);
    v.push(s[0] - 0);
    v.push(s[3] - s[1]);
    v.push(s[1] - 0);
    println!("{}", v.iter().min().unwrap());
}