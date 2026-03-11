fn main() {
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();

        let v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

        if v[0] == 0 && v[1] == 0 {
            break;
        }

        println!("{}", v[0] + v[1]);
    }
}