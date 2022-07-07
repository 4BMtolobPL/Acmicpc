fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut board: Vec<String> = Vec::new();
    for _ in 0..v[0] {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        board.push(buf.trim().to_string());
    };
}