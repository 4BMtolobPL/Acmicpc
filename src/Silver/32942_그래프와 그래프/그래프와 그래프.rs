use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());
    let (a, b, c): (i32, _, _) = (
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );

    for i in 1..=10 {
        let answer: Vec<_> = (1..=10)
            .filter(|y| (i * a) + (y * b) == c)
            .map(|x| x.to_string())
            .collect();
        if answer.is_empty() {
            println!("0");
        } else {
            println!("{}", answer.join(" "));
        }
    }
}
