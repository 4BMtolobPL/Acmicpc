use std::io::{stdin, Read};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let v: Vec<_> = buf
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let mut stack = vec![(0, v[0])];
    for (index, value) in v.iter().enumerate() {
        match index {
            0 => continue,
            1 => stack.push((v[index], v[index] + v[index - 1])),
            _ => {
                let two_step = stack[index - 2];
                let one_step = stack[index - 1];
                stack.push((two_step.0.max(two_step.1) + value, one_step.0 + value));
            }
        }
    }

    let last = stack.pop().unwrap();
    println!("{}", last.0.max(last.1));
}
