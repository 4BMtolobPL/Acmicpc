use std::collections::VecDeque;
use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();

    let mut first_row = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap());
    let n = first_row.next().unwrap();
    let m = first_row.next().unwrap();

    let map: Vec<Vec<i32>> = lines
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
    let mut distance = Vec::with_capacity(n);
    for _ in 0..n {
        distance.push(vec![-1; m]);
    }

    for (i, row) in map.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value == 0 {
                distance[i][j] = 0;
            }
        }
    }

    let mut queue = VecDeque::new();
    let start_point = find_start_point(&map);
    queue.push_back(start_point);
    distance[start_point.0][start_point.1] = 0;

    while let Some((x, y)) = queue.pop_front() {
        let now = distance[x][y];

        if x > 0 && distance[x - 1][y] == -1 {
            distance[x - 1][y] = if map[x - 1][y] == 0 {
                0
            } else {
                queue.push_back((x - 1, y));
                now + 1
            };
        }
        if x < n - 1 && distance[x + 1][y] == -1 {
            distance[x + 1][y] = if map[x + 1][y] == 0 {
                0
            } else {
                queue.push_back((x + 1, y));
                now + 1
            };
        }
        if y > 0 && distance[x][y - 1] == -1 {
            distance[x][y - 1] = if map[x][y - 1] == 0 {
                0
            } else {
                queue.push_back((x, y - 1));
                now + 1
            };
        }
        if y < m - 1 && distance[x][y + 1] == -1 {
            distance[x][y + 1] = if map[x][y + 1] == 0 {
                0
            } else {
                queue.push_back((x, y + 1));
                now + 1
            };
        }
    }

    println!(
        "{}",
        distance
            .iter()
            .map(|line| line
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" "))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn find_start_point(map: &[Vec<i32>]) -> (usize, usize) {
    for (i, row) in map.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value == 2 {
                return (i, j);
            }
        }
    }

    panic!("No point found")
}
