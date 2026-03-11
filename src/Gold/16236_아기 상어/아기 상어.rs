use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_whitespace().map(|x| x.parse().unwrap());

    let n: usize = iter.next().unwrap();

    let mut v: Vec<Vec<usize>> = vec![vec![0; n]; n];

    let mut baby_shark = (0, 0);
    let mut shark_size = 2;
    let mut fishs = [0; 7];

    for (i, value) in v.iter_mut().enumerate() {
        for (j, x) in value.iter_mut().enumerate() {
            let next = iter.next().unwrap();
            if next == 9 {
                baby_shark = (i, j);
                *x = 0;
            } else {
                fishs[next] += 1;
                *x = next;
            }
        }
    }

    let mut count_ate = 0;
    let mut time = 0;
    while contains_small(shark_size, &fishs) {
        if let Some((spend_time, next_position)) = bfs(baby_shark, shark_size, &mut v, &mut fishs) {
            count_ate += 1;
            time += spend_time;
            baby_shark = next_position;
            if count_ate == shark_size {
                shark_size += 1;
                count_ate = 0;
            }
        } else {
            break;
        }
    }

    println!("{}", time);
}

fn contains_small(size: usize, fishs: &[i32; 7]) -> bool {
    for i in fishs.iter().skip(1).take(size) {
        if *i > 0 {
            return true;
        }
    }
    false
}

fn bfs(
    position: (usize, usize),
    size: usize,
    v: &mut Vec<Vec<usize>>,
    fishs: &mut [i32; 7],
) -> Option<(i32, (usize, usize))> {
    let len = v.len();
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; v.len()]; v.len()];
    queue.push_back(position);
    visited[position.0][position.1] = true;

    let mut next_position = (21, 21);

    let mut time = 0;
    while !queue.is_empty() {
        time += 1;
        for _ in 0..queue.len() {
            let (x, y) = queue.pop_front().unwrap();

            let mut closer = |next_x: usize, next_y| {
                let n = v[next_x][next_y];
                if n > 0 && n < size {
                    if next_position > (next_x, next_y) {
                        next_position = (next_x, next_y);
                    }
                } else if (n == size || n == 0) && !visited[next_x][next_y] {
                    visited[next_x][next_y] = true;
                    queue.push_back((next_x, next_y));
                }
            };

            if x > 0 {
                closer(x - 1, y);
            }
            if y > 0 {
                closer(x, y - 1);
            }
            if x < len - 1 {
                closer(x + 1, y);
            }
            if y < len - 1 {
                closer(x, y + 1);
            }
        }
        if next_position.0 != 21 {
            fishs[v[next_position.0][next_position.1]] -= 1;
            v[next_position.0][next_position.1] = 0;
            return Some((time, next_position));
        }
    }
    None
}