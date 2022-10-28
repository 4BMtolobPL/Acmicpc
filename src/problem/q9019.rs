use std::io::Write;
use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter, Read},
};

#[allow(dead_code)]
pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut numbers = buf.split_whitespace().map(|x| x.parse().unwrap());

    let test_case: usize = numbers.next().unwrap();

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);

    for _ in 0..test_case {
        let (src, dst) = (numbers.next().unwrap(), numbers.next().unwrap());
        if src == dst {
            println!();
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back(src);
        let mut visited = [None; 10000];
        visited[src] = Some((src, ' '));

        let nexts = |x| {
            [
                (x * 2) % 10000,
                if x == 0 { 9999 } else { x - 1 },
                (x * 10) % 10000 + (x / 1000),
                (x % 10) * 1000 + (x / 10),
            ]
        };

        let commands = ['D', 'S', 'L', 'R'];

        while let Some(value) = queue.pop_front() {
            for (index, next) in nexts(value).into_iter().enumerate() {
                if visited[next].is_none() {
                    visited[next] = Some((value, commands[index]));
                    queue.push_back(next);
                }
            }
            if visited[dst].is_some() {
                break;
            }
        }

        let mut v = Vec::new();
        let mut from = dst;
        while from != src {
            v.push(visited[from].unwrap().1);
            from = visited[from].unwrap().0;
        }
        writeln!(out, "{}", v.iter().rev().collect::<String>()).unwrap();
    }
}
