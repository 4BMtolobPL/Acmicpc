fn main() {
    let n: i32 = get_line().trim().parse().unwrap();
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(get_line().trim().to_string());
    }

    v.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    v.dedup();

    println!("{}", v.join("\n"));
}

fn get_line() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    return buf;
}