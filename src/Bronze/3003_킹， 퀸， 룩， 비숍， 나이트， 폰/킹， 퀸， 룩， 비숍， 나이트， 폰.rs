use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let s = vec![1, 1, 2, 2, 2, 8];
    let v: Vec<i32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();
    println!(
        "{}",
        s.iter()
            .zip(v)
            .map(|(lhs, rhs)| (lhs - rhs).to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}