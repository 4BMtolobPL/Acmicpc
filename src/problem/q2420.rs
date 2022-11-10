fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let (n, m): (i32, _) = (iter.next().unwrap(), iter.next().unwrap());

    println!("{}", n.abs_diff(m));
}
