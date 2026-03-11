fn main() {
    loop {
        let mut buf = String::new();
        let _ = match std::io::stdin().read_line(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
            },
            Err(_) => break,
        };
        let v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

        println!("{}", v[0] + v[1]);
    }
}