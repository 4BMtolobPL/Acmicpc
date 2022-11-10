use std::io::stdin;

fn main() {
    let mut list = [0; 1001];
    list[1] = 1;
    list[2] = 3;
    for index in 3..=1000 {
        list[index] = (list[index - 2] * 2 + list[index - 1]) % 10007;
    }

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    println!("{}", list[n]);
}
