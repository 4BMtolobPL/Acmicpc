use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut numbers = buf.split_whitespace().map(|x| x.parse().unwrap());

    let test_case: usize = numbers.next().unwrap();

    for _ in 0..test_case {
        let (src, dst) = (numbers.next().unwrap(), numbers.next().unwrap());
        if src == dst {
            println!();
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back((src, Vec::new()));
        let mut visited = [false; 10000];
        visited[src] = true;

        let closer = |x| {
            [
                (x * 2) % 10000,
                if x == 0 { 9999 } else { x - 1 },
                (x * 10) % 10000 + (x / 1000),
                (x % 10) * 1000 + (x / 10),
            ]
        };

        let commands = ['D' as u8, 'S' as u8, 'L' as u8, 'R' as u8];

        'outer: while let Some((value, command)) = queue.pop_front() {
            for (index, next) in closer(value).into_iter().enumerate() {
                if next == dst {
                    println!("{}{}", String::from_utf8(command).unwrap(), commands[index] as char);
                    break 'outer;
                } else if !visited[next] {
                    visited[next] = true;
                    let mut c = command.clone();
                    c.push(commands[index]);
                    queue.push_back((next, c));
                }
            }
        }
    }
}
