use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let mut graph = vec![Vec::new(); n + 1];
    for line in lines {
        let mut iter = line.split_whitespace().map(|x| x.parse().unwrap());
        let lhs: usize = iter.next().unwrap();
        let rhs = iter.next().unwrap();

        graph[lhs].push(rhs);
        graph[rhs].push(lhs);
    }

    let mut stack: Vec<usize> = Vec::new();
    let mut visited = HashSet::new();
    let mut count = 0;
    for first in 1..=n {
        if !visited.contains(&first) {
            stack.push(first);
            count += 1;

            while !stack.is_empty() {
                let last = stack.pop().unwrap();
                if !visited.contains(&last) {
                    for i in &graph[last] {
                        stack.push(*i);
                    }
                    visited.insert(last);
                }
            }
        }
    }

    println!("{}", count);
}
