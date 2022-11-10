use std::io::{prelude::*, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());

    let mut res = String::new();
    let n: i32 = iter.next().unwrap();
    let mut stack = Vec::new();
    let mut next = iter.next().unwrap();

    for i in 1..=n {
        stack.push(i);
        res.push_str("+\n");
        if stack.last() == None {
            continue;
        }
        while let Some(i) = stack.last() {
            if *i == next {
                stack.pop();
                res.push_str("-\n");
                next = if let Some(i) = iter.next() { i } else { break };
            } else {
                break;
            }
        }
    }

    if !stack.is_empty() {
        res.clear();
        res.push_str("NO");
    }

    println!("{}", res.trim());
}
