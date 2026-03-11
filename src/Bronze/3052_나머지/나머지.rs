fn main() {
    let mut v:Vec<i32> = Vec::new();
    for _ in 0..10 {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        v.push(buf.trim().parse::<i32>().unwrap() % 42);
    }
    v.sort();
    v.dedup();
    println!("{}", v.len());
}