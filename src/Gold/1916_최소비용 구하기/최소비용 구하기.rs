use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let m: usize = lines.next().unwrap().parse().unwrap();

    let mut bus_lines: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n + 1];

    for line in lines.by_ref().take(m) {
        let mut iter = line.split_ascii_whitespace();
        let from: usize = iter.next().unwrap().parse().unwrap();
        let to: usize = iter.next().unwrap().parse().unwrap();
        let cost: i32 = iter.next().unwrap().parse().unwrap();

        bus_lines[from].push((to, cost));
    }

    let (start, end): (usize, usize) = {
        let mut iter = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let mut distance = vec![i32::MAX; n + 1];
    distance[start] = 0;
    let mut visited = vec![false; n + 1];

    let mut now = Some(start);

    while !visited[end] {
        if let Some(from) = now {
            for (to, cost) in bus_lines[from].iter() {
                if distance[from] + cost < distance[*to] {
                    distance[*to] = distance[from] + cost;
                }
            }

            visited[from] = true;

            if let Some(min) = distance
                .iter()
                .enumerate()
                .filter(|(index, _)| !visited[*index])
                .min_by(|&lhs, &rhs| lhs.1.cmp(rhs.1))
            {
                now = Some(min.0);
            }
        } else {
            break;
        }
    }

    println!("{}", distance[end]);
}
