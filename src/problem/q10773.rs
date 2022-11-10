use std::io::{stdin, Read};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut iter = buf.split_whitespace().map(|x| x.parse::<i32>().unwrap());
    iter.next();

    let mut stack = Vec::new();

    for i in iter {
        if i == 0 {
            stack.pop();
        } else {
            stack.push(i);
        }
    }

    println!("{}", stack.iter().sum::<i32>());
}
