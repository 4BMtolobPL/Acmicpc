fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let test_case: i32 = buf.trim().parse().unwrap();

    for _ in 0..test_case {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

        println!("{}", v[0] + v[1]);
    }
}