#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut target: i32 = buf.trim().parse().unwrap();
    let mut now = 0;
    while target > 0 {
        now += 1;
        if now.to_string().contains("666") {
            target -= 1;
        }
    }
    println!("{}", now);
}