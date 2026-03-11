use std::collections::VecDeque;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut lines = buf.lines();

    let (n, m): (usize, usize) = {
        let mut iter = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };

    let v: Vec<Vec<bool>> = lines
        .map(|line| line.chars().map(|c| c == '0').collect())
        .collect();

    println!("{}", bfs(n, m, &v));
}

fn bfs(n: usize, m: usize, v: &[Vec<bool>]) -> i32 {
    // (x, y, b) b: 벽을 뚫을 기회가 남아있는지 여부
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, true));

    // 0: 미도달, 1: 벽을 부수지 않고 도달, 2: 벽을 부수고 도달
    let mut visited = vec![vec![0; m]; n];
    visited[0][0] = 1;

    let mut count = 0;
    while !queue.is_empty() {
        count += 1;

        for _ in 0..queue.len() {
            let (x, y, b) = queue.pop_front().unwrap();

            if x == n - 1 && y == m - 1 {
                return count;
            }

            let mut next_closer = |next_x: usize, next_y: usize| {
                if visited[next_x][next_y] == 1 {
                    // 이미 벽을 부수지 않고 해당 위치에 온 경우
                    return;
                } else if visited[next_x][next_y] == 2 && !b {
                    // 이전에 벽을 부수고 도달한적이 있고, 또 벽을 부수고 도달한경우
                    return;
                }

                // 아에 해당 위치에 도달한적이 없거나
                // 벽을 부수지 않고 도달한적이 없고, 이전에 벽을 부수고 도달한적이 있으며 벽을 부수지 않고 다시 도달한경우
                if v[next_x][next_y] {
                    // 이동할 수 있는 곳인경우
                    queue.push_back((next_x, next_y, b));
                    visited[next_x][next_y] = if b { 1 } else { 2 };
                } else if b {
                    // 벽이지만 부술 수 있는 경우
                    queue.push_back((next_x, next_y, false));
                    visited[next_x][next_y] = 2;
                }
            };

            if x < n - 1 {
                next_closer(x + 1, y);
            }
            if y < m - 1 {
                next_closer(x, y + 1);
            }
            if x > 0 {
                next_closer(x - 1, y);
            }
            if y > 0 {
                next_closer(x, y - 1);
            }
        }
    }

    -1
}
