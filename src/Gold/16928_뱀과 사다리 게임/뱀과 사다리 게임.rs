use std::{
    collections::{HashMap, VecDeque},
    io::{stdin, Read},
};

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();

    buf.clear();
    stdin().read_to_string(&mut buf).unwrap();

    let mut v: HashMap<usize, usize> = HashMap::new();
    for line in buf.lines() {
        let mut iter = line.split_whitespace().map(|x| x.parse().unwrap());
        v.insert(iter.next().unwrap(), iter.next().unwrap());
    }

    let mut queue = VecDeque::new();
    queue.push_back(1);
    let mut visited = vec![false; 101];
    visited[0] = true;
    visited[1] = true;

    let mut counter = 1;
    'outer: while !queue.is_empty() {
        for _ in 0..queue.len() {
            let front = queue.pop_front().unwrap();

            for i in 1..=6 {
                let next = if v.contains_key(&(front + i)) {
                    v.get(&(front + i)).unwrap().to_owned()
                } else {
                    front + i
                };

                if next == 100 {
                    break 'outer;
                }

                if !visited[next] {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }
        counter += 1;
    }

    println!("{}", counter);
}