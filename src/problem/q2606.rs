use std::{
    collections::HashSet,
    io::{stdin, Read},
    vec,
};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let computer: usize = lines.next().unwrap().parse().unwrap();
    lines.next();

    let mut computers = vec![Vec::new(); computer + 1];

    for line in lines {
        let mut words = line.split_whitespace().map(|x| x.parse().unwrap());
        let n: usize = words.next().unwrap();
        let m = words.next().unwrap();
        computers[n].push(m);
        computers[m].push(n);
    }

    let mut stack = vec![1];
    let mut visited = HashSet::new();
    while !stack.is_empty() {
        let last = stack.pop().unwrap();
        if visited.contains(&last) {
            continue;
        }
        visited.insert(last);
        for i in &computers[last] {
            stack.push(*i);
        }
    }

    println!("{}", visited.len() - 1);
}
