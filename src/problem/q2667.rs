use std::io::{Read, Write};
use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter},
};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();

    let mut map: Vec<Vec<Option<bool>>> = Vec::new();
    for line in buf.lines().skip(1) {
        map.push(
            line.chars()
                .map(|x| if x == '1' { Some(false) } else { None })
                .collect(),
        );
    }

    let mut complex = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            match map[i][j] {
                Some(true) => continue,
                Some(false) => complex.push(bfs(&mut map, i, j)),
                None => continue,
            }
        }
    }
    complex.sort_unstable();

    let stdout = stdout();
    let mut out = BufWriter::new(stdout);
    writeln!(out, "{}", complex.len()).unwrap();
    for i in complex {
        writeln!(out, "{}", i).unwrap();
    }
}

fn bfs(map: &mut Vec<Vec<Option<bool>>>, x: usize, y: usize) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    map[x][y] = Some(true);

    let mut count = 0;
    while !queue.is_empty() {
        let (front_x, front_y) = queue.pop_front().unwrap();
        count += 1;

        let nexts = get_next(front_x, front_y, map.len());

        for (next_x, next_y) in nexts {
            if let Some(x) = map[next_x][next_y] {
                if !x {
                    map[next_x][next_y] = Some(true);
                    queue.push_back((next_x, next_y));
                }
            }
        }
    }
    count
}

fn get_next(x: usize, y: usize, len: usize) -> Vec<(usize, usize)> {
    let mut nexts = Vec::new();
    if x > 0 {
        nexts.push((x - 1, y));
    }
    if y > 0 {
        nexts.push((x, y - 1));
    }
    if x < len - 1 {
        nexts.push((x + 1, y));
    }
    if y < len - 1 {
        nexts.push((x, y + 1));
    }
    nexts
}
