fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.split_whitespace().map(|x| x.chars().rev().collect::<String>().parse().unwrap()).max().unwrap();
    println!("{}", n);
}