fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let board: Vec<String> = Vec::new();
    for i in 0..v[0] {
        board.push(Vec::new());
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        for c in buf.trim().chars() {
            board[i].push(c);
        }
    };
}