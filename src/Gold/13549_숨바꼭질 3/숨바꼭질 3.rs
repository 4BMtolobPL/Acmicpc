use std::collections::VecDeque;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let (n, k): (usize, usize) = {
        let mut iter = buf.split_ascii_whitespace().map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    println!("{}", zero_one_bfs(n, k));
}

fn zero_one_bfs(from: usize, to: usize) -> usize {
    if from >= to {
        return from - to;
    }

    let mut distance = [usize::MAX; 100001];
    distance[from] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(from);

    while let Some(parent) = queue.pop_front() {
        let mut get_next = |parent, child, weight| {
            if distance[child] > distance[parent] + weight {
                distance[child] = distance[parent] + weight;
                if weight == 0 {
                    queue.push_front(child);
                } else {
                    queue.push_back(child);
                }
            }
        };

        if parent <= 50000 {
            let child = parent * 2;
            get_next(parent, child, 0);
            if child == to {
                break;
            }
        }
        if parent > 0 {
            let child = parent - 1;
            get_next(parent, child, 1);
            if child == to {
                break;
            }
        }
        if parent < 100000 {
            let child = parent + 1;
            get_next(parent, child, 1);
            if child == to {
                break;
            }
        }
    }

    distance[to]
}
