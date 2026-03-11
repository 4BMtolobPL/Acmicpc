use std::collections::VecDeque;
use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());

    let n: usize = iter.next().unwrap();

    let mut graph = vec![vec![]; n + 1];

    for _ in 0..(n + 3) {
        let lhs = iter.next().unwrap();
        let rhs = iter.next().unwrap();

        graph[lhs].push(rhs);
        graph[rhs].push(lhs);
    }

    let start = graph.iter().position(|x| x.len() == 1).unwrap();

    let mut queue = VecDeque::new();
    let mut visited = vec![false; n + 1];
    let mut answer = [0; 6];

    queue.push_back(start);
    visited[start] = true;
    answer[5] = start;

    while let Some(node) = queue.pop_front() {
        if graph[node].len() == 3 {
            if answer[1] == 0 {
                answer[1] = node;
            } else if answer[3] == 0 {
                if answer[1] > node {
                    answer[3] = answer[1];
                    answer[1] = node;
                } else {
                    answer[3] = node;
                }
            } else {
                answer[0] = node;
                break;
            }
        } else if graph[node].len() == 4 {
            if answer[4] == 0 {
                answer[4] = node;
            } else {
                answer[2] = node;
            }
        }

        for i in graph[node].iter() {
            if !visited[*i] {
                visited[*i] = true;

                if graph[*i].len() > 2 {
                    queue.push_back(*i);
                } else {
                    queue.push_front(*i);
                }
            }
        }
    }

    let mut out = String::new();
    let mut iter = answer.iter();
    write!(out, "{}", iter.next().unwrap()).unwrap();
    for i in iter {
        write!(out, " {i}").unwrap();
    }

    println!("{out}");
}
