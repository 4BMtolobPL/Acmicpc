use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n = lines.next().unwrap().parse().unwrap();

    let mut out = Vec::new();
    let mut queue = VecDeque::new();
    for _ in 0..n {
        let order: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
        match order.first() {
            Some(x) => match *x {
                "push" => queue.push_back(order[1]),
                "pop" => {
                    let a = match queue.pop_front() {
                        Some(x) => x.to_string(),
                        None => "-1".to_string(),
                    };
                    out.push(a);
                }
                "size" => {
                    let len = queue.len().to_string();
                    out.push(len)
                }
                "empty" => out.push(if queue.is_empty() {
                    "1".to_string()
                } else {
                    "0".to_string()
                }),
                "front" => {
                    let a = match queue.front() {
                        Some(x) => x.to_string(),
                        None => "-1".to_string(),
                    };
                    out.push(a);
                }
                "back" => {
                    let a = match queue.back() {
                        Some(x) => x.to_string(),
                        None => "-1".to_string(),
                    };
                    out.push(a);
                }
                _ => continue,
            },
            None => continue,
        }
    }

    println!("{}", out.join("\n"));
}
