use std::{io::{stdin, Read}, collections::VecDeque, fmt::format};

pub fn solve() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut numbers = buf.split_whitespace().map(|x| x.parse().unwrap());

    let test_case: usize = numbers.next().unwrap();

    for _ in 0..test_case {
        let (src, dst) = (numbers.next().unwrap(), numbers.next().unwrap());
        if src == dst {
            println!("");
            continue;
        }

        let mut queue = VecDeque::from([(src, String::new())]);
        let mut visited = [false; 10000];
        visited[src] = true;
        let commands = ['D', 'S', 'L', 'R'];

        let mut counter = 0;
        while let Some((x, command)) = queue.pop_front() {
            println!("queue: {:?}", queue);
            let nexts = [(x * 2) % 10000,
            if x == 0 {9999} else {x - 1},
            (x * 10) % 10000 + (x / 1000),
            (x % 10) * 1000 + (x / 10)];
            println!("nexts: {:?}", nexts);

            if counter > 3 {
                break;
            }
            counter += 1;

            for index in 0..4 {
                if !visited[nexts[index]] {
                    if nexts[index] == dst {
                        println!("{}{}", command, commands[index]);
                        queue.clear();
                    } else {
                        visited[nexts[index]] = true;
                        queue.push_back((nexts[index], format!("{}{}", command, commands[index])));
                        continue;
                    }
                }
                break;
            }
        }
    }
}