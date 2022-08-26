use std::io::stdin;

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();
    let mut five = n / 5;
    let mut etc = n % 5;
    let mut out = -1;
    while five >= 0 {
        if etc % 3 == 0 {
            out = five + (etc / 3);
            break;
        } else {
            five -= 1;
            etc += 5;
        }
    }

    println!("{}", out);
}
