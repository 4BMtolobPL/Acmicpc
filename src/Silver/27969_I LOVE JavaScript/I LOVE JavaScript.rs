use std::cmp::PartialEq;
use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    let mut stack: Vec<Ason> = Vec::new();

    for word in buf.split_whitespace() {
        if word.eq("[") {
            stack.push(Ason::Open);
        } else if word.eq("]") {
            let mut sum = 8;
            let mut x = stack.pop().unwrap();

            while let Ason::Number(i) = x {
                sum += i;
                x = stack.pop().unwrap();
            }

            stack.push(Ason::Number(sum));
        } else {
            let result = word.parse::<usize>();
            if result.is_ok() {
                stack.push(Ason::Number(8));
            } else {
                stack.push(Ason::Number(word.len() + 12))
            }
        }
    }

    if let Ason::Number(n) = stack.pop().unwrap() {
        println!("{}", n);
    };
}

enum Ason {
    Number(usize),
    Open,
}
