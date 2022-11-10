use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse().unwrap();

    let mut count = 0;
    for i in 1..=n {
        if i % 5 == 0 {
            count += 1;
        }
        if i % 25 == 0 {
            count += 1;
        }
        if i % 125 == 0 {
            count += 1;
        }
    }

    println!("{}", count);
}
