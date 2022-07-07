pub fn solve() {
    let v: Vec<i32> = get_line().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut board: Vec<String> = Vec::new();
    for _ in 0..v[0] {
        board.push(get_line());
    }
    

}

fn get_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}