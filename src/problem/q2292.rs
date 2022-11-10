use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse().unwrap();

    let mut count = 1;
    while count * (count - 1) * 3 + 1 < n {
        count += 1;
    }

    println!("{}", count);
}
