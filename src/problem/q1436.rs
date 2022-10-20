#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let target: usize = buf.trim().parse().unwrap();

    let mut v = Vec::with_capacity(10000);
    for i in 666..2666800 {
        let mut j = i;
        while j > 665 {
            if j % 1000 == 666 {
                v.push(i);
                break;
            } else {
                j /= 10;
            }
        }
    }
    println!("{}", v[target - 1]);
}