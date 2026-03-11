fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let sum: u32 = buf.trim().chars().map(|x| x.to_digit(10).unwrap()).sum();

    println!("{}", sum);
}
