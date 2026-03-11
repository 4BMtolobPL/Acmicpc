use std::fmt::Write;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let iter = lines.map(|line| {
        let mut x = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        (x.next().unwrap(), x.next().unwrap())
    });

    let mut tree: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for (l, r) in iter {
        tree[l].push(r);
        tree[r].push(l);
    }

    let mut stack = vec![1];
    let mut parents = vec![0; n + 1];
    parents[1] = 1;

    while let Some(parent) = stack.pop() {
        for child in tree[parent].iter() {
            if parents[*child] == 0 {
                parents[*child] = parent;
                stack.push(*child);
            }
        }
    }

    let mut out = String::new();
    for i in parents.iter().skip(2) {
        writeln!(out, "{i}").unwrap();
    }

    print!("{out}");
}
